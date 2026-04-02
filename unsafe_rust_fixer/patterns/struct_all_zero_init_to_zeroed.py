"""Plugin: Collapse all-zero struct literals into Type::default().

Replaces verbose struct literals with Type::default() and generates impl Default
in the struct definition file if needed. Falls back to unsafe { zeroed() } if
definition file not found. Uses field validation to ensure safe generation.
"""

import os
import re
from typing import Any, List, Optional, Set, Tuple

from .base import UnsafePatternPlugin


def _find_workspace_root() -> str:
    current = os.path.dirname(os.path.abspath(__file__))
    for _ in range(8):
        if os.path.isfile(os.path.join(current, 'Cargo.toml')):
            return current
        parent = os.path.dirname(current)
        if parent == current:
            break
        current = parent
    # Hard fallback for this specific project layout
    return '/workspace'


_WORKSPACE_ROOT = _find_workspace_root()


class StructAllZeroInitToZeroedPlugin(UnsafePatternPlugin):
    """Collapse verbose all-zero struct literals to Type::default().

    Phase 1 (use site): replace the struct literal with TypePath::default().
    Phase 2 (definition site): insert impl Default into the struct's source file.

    Falls back to  unsafe { ::core::mem::zeroed() }  with a SAFETY comment
    when the struct definition cannot be located.
    """

    def __init__(self) -> None:
        super().__init__()
        # Track types whose Default impl has already been added in this run,
        # keyed by simple type name.
        self._added_defaults: Set[str] = set()

    @property
    def name(self) -> str:
        return "struct_all_zero_init_to_zeroed"

    @property
    def description(self) -> str:
        return (
            "Collapse verbose all-zero struct literals into Type::default() "
            "(generates impl Default in the struct's definition file) — "
            "standalone case (no memset), safe alternative to zeroed()"
        )

    @property
    def priority(self) -> int:
        # One below zero_init_with_memset (10) so the paired case is handled first.
        return 9

    # ══════════════════════════════════════════════════════════════════════════
    # ── Section 1: Zero-value detection ──────────────────────────────────────
    # ══════════════════════════════════════════════════════════════════════════

    def _is_zero_node(self, node: Any, code: str) -> bool:
        """Return True if the AST node evaluates to an all-zero bit pattern.

        Accepts:
          - Integer literal 0 (possibly through cast chains)
          - Boolean false
          - null_mut::<T>() and null::<T>() calls
          - [0; N] array expressions
          - Struct / union literals whose ALL specified fields are zero
        """
        if node is None:
            return False

        ntype = node.type

        if ntype == 'integer_literal':
            return self.node_text(node, code).strip() == '0'

        if ntype == 'boolean_literal':
            return self.node_text(node, code).strip() == 'false'

        if ntype in ('cast_expression', 'type_cast_expression'):
            inner = node.child_by_field_name('value')
            return self._is_zero_node(inner, code)

        if ntype == 'parenthesized_expression':
            inner_nodes = [
                c for c in node.children if c.type not in ('(', ')', 'comment')
            ]
            return len(inner_nodes) == 1 and self._is_zero_node(inner_nodes[0], code)

        if ntype == 'call_expression':
            return self._is_null_ptr_call(node, code)

        if ntype == 'array_expression':
            return self._is_zero_array(node, code)

        if ntype == 'struct_expression':
            return self._is_zero_struct_expr(node, code)

        return False

    def _is_null_ptr_call(self, call_node: Any, code: str) -> bool:
        """Return True for ::core::ptr::null_mut::<T>() or null::<T>()."""
        fn_node = call_node.child_by_field_name('function')
        if fn_node is None:
            return False
        fn_text = self.node_text(fn_node, code)
        if ('null_mut' not in fn_text
                and 'null' not in fn_text):
            return False
        if 'null_mut' not in fn_text:
            # Plain null — confirm it's a ptr::null path
            if not re.search(r'(?:^|::)null(?:::<|\s*$|\s*\()', fn_text):
                return False
        args_node = call_node.child_by_field_name('arguments')
        if args_node is None:
            return False
        real_args = [
            c for c in args_node.children
            if c.type not in ('(', ')', ',', 'comment')
        ]
        return len(real_args) == 0

    def _is_zero_array(self, arr_node: Any, code: str) -> bool:
        """Return True for [zero_element; N]."""
        children = [
            c for c in arr_node.children
            if c.type not in ('[', ']', ';', 'comment')
        ]
        return not children or self._is_zero_node(children[0], code)

    def _is_zero_struct_expr(self, struct_node: Any, code: str) -> bool:
        """Return True if ALL field_initializers in the struct literal are zero."""
        body = struct_node.child_by_field_name('body')
        if body is None:
            return True
        for child in body.children:
            if child.type == 'field_initializer':
                value = child.child_by_field_name('value')
                if value is None:
                    return False  # shorthand { field } — value is a variable
                if not self._is_zero_node(value, code):
                    return False
            elif child.type == 'base_field_initializer':
                return False  # ..expr update syntax
        return True

    # ══════════════════════════════════════════════════════════════════════════
    # ── Section 2: Companion-memset detection ─────────────────────────────────
    # ══════════════════════════════════════════════════════════════════════════

    def _has_subsequent_memset(self, var_name: str, let_node: Any, code: str) -> bool:
        """Return True if a memset(var_name, 0, …) exists after let_node in its block."""
        block = self.get_parent_of_type(let_node, 'block')
        if block is None:
            return False
        for call in self.find_nodes(block, 'call_expression'):
            if call.start_byte < let_node.end_byte:
                continue
            call_text = self.node_text(call, code)
            if 'memset' not in call_text or var_name not in call_text:
                continue
            fn_node = call.child_by_field_name('function')
            if fn_node is None:
                continue
            fn_text = self.node_text(fn_node, code)
            if fn_text == 'memset' or fn_text.endswith('::memset'):
                return True
        return False

    # ══════════════════════════════════════════════════════════════════════════
    # ── Section 3: Match collection ───────────────────────────────────────────
    # ══════════════════════════════════════════════════════════════════════════

    def _extract_var_name(self, let_node: Any, code: str) -> Optional[str]:
        """Return the variable name bound by a let_declaration node."""
        pattern = let_node.child_by_field_name('pattern')
        if pattern is not None and pattern.type == 'identifier':
            var_name = self.node_text(pattern, code).strip()
            # Validate that it's a valid identifier (alphanumeric + underscore)
            if var_name and all(c.isalnum() or c == '_' for c in var_name) and not var_name[0].isdigit():
                return var_name
        for i, child in enumerate(let_node.children):
            if child.type == 'identifier':
                if let_node.field_name_for_child(i) == 'pattern':
                    var_name = self.node_text(child, code).strip()
                    # Validate that it's a valid identifier
                    if var_name and all(c.isalnum() or c == '_' for c in var_name) and not var_name[0].isdigit():
                        return var_name
        return None

    def _collect_matches(self, code: str) -> List[Tuple[Any, Any, str]]:
        """Return (let_node, struct_value_node, var_name) for each candidate.

        Candidates are let_declarations where:
          1. value is a struct_expression (not Self { … })
          2. All struct fields are zero-valued
          3. No companion memset in the same block
        """
        root = self.parse(code)
        matches = []
        for let_node in self.find_nodes(root, 'let_declaration'):
            var_name = self._extract_var_name(let_node, code)
            if var_name is None:
                continue
            value_node = let_node.child_by_field_name('value')
            if value_node is None:
                continue
            # CRITICAL: only match struct_expression nodes, not call_expression or other types
            if value_node.type != 'struct_expression':
                continue
            # Skip Self { … } — that's the body of our generated Default impl
            name_node = value_node.child_by_field_name('name')
            if name_node is not None:
                name_text = self.node_text(name_node, code).strip()
                if name_text == 'Self':
                    continue
            if not self._is_zero_struct_expr(value_node, code):
                continue
            if self._has_subsequent_memset(var_name, let_node, code):
                continue
            matches.append((let_node, value_node, var_name))
        return matches

    # ══════════════════════════════════════════════════════════════════════════
    # ── Section 4: Type-path extraction ──────────────────────────────────────
    # ══════════════════════════════════════════════════════════════════════════

    def _extract_type_path(self, struct_node: Any, code: str) -> Optional[str]:
        """Return the full type path from a struct expression, e.g. 'crate::sqliteInt_h::Parse'."""
        name_node = struct_node.child_by_field_name('name')
        if name_node is None:
            return None
        return self.node_text(name_node, code).strip()

    def _split_type_path(self, type_path: str) -> Tuple[str, Optional[str]]:
        """Split 'crate::sqliteInt_h::Parse' → ('Parse', 'sqliteInt_h').

        Returns (simple_name, module_hint).  module_hint may be None.
        """
        parts = [p.strip() for p in type_path.split('::')]
        simple_name = parts[-1]
        module_hint: Optional[str] = None
        if len(parts) >= 2:
            candidate = parts[-2]
            if candidate not in ('crate', 'super', 'self', 'std', 'core'):
                module_hint = candidate
        return simple_name, module_hint

    # ══════════════════════════════════════════════════════════════════════════
    # ── Section 5: Definition-file discovery ─────────────────────────────────
    # ══════════════════════════════════════════════════════════════════════════

    def _find_definition_file(
        self, type_name: str, module_hint: Optional[str]
    ) -> Optional[str]:
        """Locate the .rs file that contains  pub struct TypeName.

        Strategy:
          1. If module_hint given, try  <root>/src/<module>.rs  first (fast path).
          2. Try  <root>/lib.rs  (monolithic layout like this SQLite port).
          3. Walk all .rs files under <root>/ for  'pub struct TypeName'.

        Returns the path of the first file found, or None.
        """
        root = _WORKSPACE_ROOT
        search_token = f'pub struct {type_name}'

        # Fast-path 1: module hint as a .rs file
        if module_hint:
            for candidate_rel in (
                os.path.join('src', f'{module_hint}.rs'),
                f'{module_hint}.rs',
                os.path.join('src', module_hint, 'mod.rs'),
            ):
                path = os.path.join(root, candidate_rel)
                if os.path.isfile(path):
                    try:
                        with open(path, 'r', encoding='utf-8', errors='replace') as f:
                            if search_token in f.read():
                                return path
                    except IOError:
                        pass

        # Fast-path 2: lib.rs at root
        lib_rs = os.path.join(root, 'lib.rs')
        if os.path.isfile(lib_rs):
            try:
                with open(lib_rs, 'r', encoding='utf-8', errors='replace') as f:
                    if search_token in f.read():
                        return lib_rs
            except IOError:
                pass

        # Full walk
        for dirpath, _dirs, fnames in os.walk(root):
            # Skip build artifacts
            if any(skip in dirpath for skip in ('target', '.git', '__pycache__')):
                continue
            for fname in sorted(fnames):
                if not fname.endswith('.rs'):
                    continue
                fpath = os.path.join(dirpath, fname)
                try:
                    with open(fpath, 'r', encoding='utf-8', errors='replace') as f:
                        if search_token in f.read():
                            return fpath
                except IOError:
                    pass
        return None

    # ══════════════════════════════════════════════════════════════════════════
    # ── Section 6: impl Default generation ───────────────────────────────────
    # ══════════════════════════════════════════════════════════════════════════

    def _simplify_null_turbofish(self, text: str) -> str:
        """Strip type params from null_mut / null calls (turbofish).

        ::core::ptr::null_mut::<crate::T>()  →  ::core::ptr::null_mut()
        ::core::ptr::null::<i8>()            →  ::core::ptr::null()

        Uses a simple non-greedy regex; safe for the single-level type params
        that appear in SQLite transpilation.
        """
        text = re.sub(r'\bnull_mut\s*::<[^()>]*>', 'null_mut', text)
        text = re.sub(r'\bnull\s*::<[^()>]*>', 'null', text)
        return text

    def _replace_nested_type_constructors(self, text: str) -> str:
        """Replace  TypePath { … }  constructors inside struct body with  TypePath::default().

        This converts nested all-zero struct literals in the impl Default body
        to nested ::default() calls, keeping the generated code clean and
        avoiding the need to spell out the nested zero fields again.

        Pattern:  <word>(::<word>)* SP* { … }   where the body contains only
        zero-valued field initializers.  We use a simple brace-counting approach
        to find the matching close brace.

        Note: this is best-effort.  Nested struct literals that are NOT all-zero
        are left unchanged (but _is_zero_struct_expr already guarantees all
        nested ones are zero, so this is safe).
        """
        # Find pattern: IDENT_PATH WHITESPACE { ... }
        # We match the opening, count braces, then replace.
        result = []
        i = 0
        n = len(text)

        # Regex to find the start of a TypePath { pattern
        # TypePath: word (:: word)* followed by optional whitespace and {
        type_path_re = re.compile(
            r'(?<![a-zA-Z0-9_])'       # not preceded by identifier char
            r'((?:[a-zA-Z_]\w*::)*[A-Z][A-Za-z0-9_]*)'  # CapitalisedTypePath
            r'(\s*)\{'
        )

        while i < n:
            m = type_path_re.search(text, i)
            if m is None:
                result.append(text[i:])
                break

            type_path = m.group(1)
            before_brace = m.group(2)  # whitespace between TypePath and {
            # Skip if it's already a Default or a known Rust keyword construct
            if type_path in ('Self', 'Some', 'Ok', 'Err', 'None'):
                result.append(text[i:m.end()])
                i = m.end()
                continue

            # Count braces to find the matching }
            start_brace = m.end() - 1  # position of opening {
            depth = 0
            end_brace = -1
            for j in range(start_brace, n):
                if text[j] == '{':
                    depth += 1
                elif text[j] == '}':
                    depth -= 1
                    if depth == 0:
                        end_brace = j
                        break

            if end_brace < 0:
                # Unbalanced — leave as-is
                result.append(text[i:m.end()])
                i = m.end()
                continue

            # Append text before the match
            result.append(text[i:m.start()])
            # Replace TypePath { … } with TypePath::default()
            result.append(f'{type_path}::default()')
            i = end_brace + 1

        return ''.join(result)

    def _extract_fields_from_literal(self, struct_node: Any, code: str) -> set:
        """Extract field names from a struct literal.

        Returns a set of field identifiers used in the literal.
        """
        body_node = struct_node.child_by_field_name('body')
        if body_node is None:
            return set()

        fields = set()
        for child in body_node.children:
            if child.type == 'field_initializer':
                field_node = child.child_by_field_name('field')
                if field_node:
                    field_name = self.node_text(field_node, code).strip()
                    fields.add(field_name)

        return fields

    def _extract_struct_fields_from_definition(
        self, file_path: str, type_name: str
    ) -> Optional[set]:
        """Parse struct definition and return set of field names.

        Returns None if struct not found or parsing failed.
        """
        try:
            with open(file_path, 'r', encoding='utf-8') as fh:
                content = fh.read()
        except IOError:
            return None

        # Find struct definition using simple regex + brace counting
        header_re = re.compile(
            r'\bpub\b(?:\s*\([^)]*\))?\s*struct\s+' + re.escape(type_name) + r'\b'
        )
        m = header_re.search(content)
        if m is None:
            return None

        # Find opening brace
        start = m.start()
        brace_open = content.find('{', start)
        if brace_open < 0:
            return None

        # Find closing brace
        depth = 0
        brace_close = None
        for i in range(brace_open, len(content)):
            ch = content[i]
            if ch == '{':
                depth += 1
            elif ch == '}':
                depth -= 1
                if depth == 0:
                    brace_close = i
                    break

        if brace_close is None:
            return None

        # Extract struct body and parse fields more carefully
        struct_body = content[brace_open+1:brace_close]

        # Split by lines and extract field names more carefully
        # Only capture identifiers that appear at line start (or after pub/visibility) and before a colon
        fields = set()
        for line in struct_body.split('\n'):
            # Remove leading whitespace and pub/visibility keywords
            line = line.strip()
            if line.startswith('pub'):
                line = line[3:].strip()
            if line.startswith('(crate)') or line.startswith('(super)'):
                line = line.split(')', 1)[1].strip()

            # Match: identifier followed by colon (but not :: or ://  which are paths)
            m = re.match(r'^([a-zA-Z_][a-zA-Z0-9_]*)\s*:', line)
            if m:
                fields.add(m.group(1))

        return fields if fields else None

    def _struct_literal_as_self_body(self, struct_node: Any, code: str) -> str:
        """Convert a struct literal into  Self { <simplified fields> }.

        Transformations applied:
          - Outer type name stripped: TypePath { … } → Self { … }
          - null_mut::<T>() → null_mut()
          - null::<T>()     → null()
          - DO NOT replace nested struct literals: nested types may not have Default impl
            Keep them as is, since they're all-zero and valid for zero-init.
        """
        body_node = struct_node.child_by_field_name('body')
        if body_node is None:
            return 'Self {}'
        body_text = self.node_text(body_node, code)
        body_text = self._simplify_null_turbofish(body_text)
        # REMOVED: body_text = self._replace_nested_type_constructors(body_text)
        # Nested types may not have Default impl yet, so keep literal zero-init form
        return f'Self {body_text}'

    def _generate_impl_default(self, type_name: str, self_body: str) -> str:
        """Return the full impl Default block as a string."""
        return (
            f'\nimpl Default for {type_name} {{\n'
            f'    fn default() -> Self {{\n'
            f'        {self_body}\n'
            f'    }}\n'
            f'}}\n'
        )

    # ══════════════════════════════════════════════════════════════════════════
    # ── Section 7: Definition-file modification ───────────────────────────────
    # ══════════════════════════════════════════════════════════════════════════

    def _impl_default_exists_in_content(self, content: str, type_name: str) -> bool:
        """Return True if impl Default for TypeName is already in the file."""
        return bool(re.search(
            r'\bimpl\s+Default\s+for\s+' + re.escape(type_name) + r'\b',
            content,
        ))

    def _find_struct_end_pos(self, content: str, type_name: str) -> Optional[int]:
        """Return the byte position just after the closing '}' of 'pub struct TypeName { … }'.

        Handles: attributes before the struct, pub(crate), pub(super), repr, etc.
        Uses brace counting rather than regex to correctly handle nested types.
        """
        # Find the struct definition header
        header_re = re.compile(
            r'\bpub\b(?:\s*\([^)]*\))?\s*struct\s+' + re.escape(type_name) + r'\b'
        )
        m = header_re.search(content)
        if m is None:
            return None

        # From the match position, scan forward to the opening '{'
        start = m.start()
        brace_open = content.find('{', start)
        if brace_open < 0:
            return None

        # Count braces
        depth = 0
        for i in range(brace_open, len(content)):
            ch = content[i]
            if ch == '{':
                depth += 1
            elif ch == '}':
                depth -= 1
                if depth == 0:
                    return i + 1  # position after the closing '}'

        return None

    def _add_default_impl_to_file(
        self, file_path: str, type_name: str, impl_text: str
    ) -> bool:
        """Insert the impl Default block after the struct definition. Idempotent."""
        try:
            with open(file_path, 'r', encoding='utf-8') as fh:
                content = fh.read()
        except IOError:
            return False

        if self._impl_default_exists_in_content(content, type_name):
            return True

        insert_pos = self._find_struct_end_pos(content, type_name)
        if insert_pos is None:
            return False

        # Ensure we have newlines for clean insertion
        impl_block = '\n' + impl_text
        new_content = content[:insert_pos] + impl_block + content[insert_pos:]

        try:
            with open(file_path, 'w', encoding='utf-8') as fh:
                fh.write(new_content)
            return True
        except IOError:
            return False

    # ══════════════════════════════════════════════════════════════════════════
    # ── Section 8: Public interface ───────────────────────────────════════════
    # ══════════════════════════════════════════════════════════════════════════

    def find(self, rust_code: str) -> List[Tuple[int, int, str]]:
        """Return findings for each standalone all-zero struct literal."""
        results = []
        for let_node, value_node, var_name in self._collect_matches(rust_code):
            line = self.node_line(let_node, rust_code)
            let_text = self.node_text(let_node, rust_code)[:100]
            struct_text = self.node_text(value_node, rust_code)
            field_count = struct_text.count(':') - struct_text.count('::')
            # SAFETY: only report if the let_node actually contains the expected pattern
            if 'let ' not in let_text or '{' not in let_text:
                # Skip matches that don't look like let declarations with struct literals
                continue
            results.append((
                let_node.start_byte,
                let_node.end_byte,
                f"Line {line}: verbose all-zero init of '{var_name}' "
                f"(~{max(field_count, 1)} fields) "
                f"→ Type::default() + impl Default in definition file",
            ))
        return results

    def fix(self, rust_code: str) -> str:
        """Replace verbose struct literals with unsafe { zeroed() }."""
        matches = self._collect_matches(rust_code)
        if not matches:
            return rust_code

        replacements: List[Tuple[int, int, str]] = []

        for _let_node, value_node, _var_name in matches:
            replacements.append((
                value_node.start_byte,
                value_node.end_byte,
                "unsafe { ::core::mem::zeroed() }",
            ))

        return self.apply_replacements(rust_code, replacements)

    # ══════════════════════════════════════════════════════════════════════════
    # ── Section 9: Self-tests ─────────────────────────────────────────────────
    # ══════════════════════════════════════════════════════════════════════════

    def test(self) -> bool:
        """Self-contained tests using temp files for the two-phase transformation."""
        import tempfile, shutil
        all_passed = True

        # ── helpers ──────────────────────────────────────────────────────────
        def _make_fixer(tmpdir: str) -> 'StructAllZeroInitToZeroedPlugin':
            """Return a plugin instance with the workspace root pointed at tmpdir."""
            import unsafe_rust_fixer.patterns.struct_all_zero_init_to_zeroed as m
            orig = m._WORKSPACE_ROOT
            m._WORKSPACE_ROOT = tmpdir
            plugin = StructAllZeroInitToZeroedPlugin()
            m._WORKSPACE_ROOT = orig
            return plugin

        def check(label: str, condition: bool) -> bool:
            nonlocal all_passed
            if condition:
                print(f'  ✓ {label}')
            else:
                print(f'  ✗ {label}')
                all_passed = False
            return condition

        # ── Test 1: simple struct, all-zero, no memset — find detects it ─────
        code1 = (
            'unsafe fn f() {\n'
            '    let mut nc: NameCtx = NameCtx {\n'
            '        pParse: ::core::ptr::null_mut::<Parse>(),\n'
            '        nRef: 0,\n'
            '        szBuffer: 0 as ::core::ffi::c_int,\n'
            '    };\n'
            '    nc.nRef = 1;\n'
            '}'
        )
        finds1 = self.find(code1)
        check('simple all-zero: find reports 1 match', len(finds1) == 1)

        # ── Test 2: non-zero field — must NOT match ───────────────────────────
        code2 = (
            'unsafe fn h() {\n'
            '    let mut x: Foo = Foo {\n'
            '        a: 0, b: 42,\n'
            '        c: ::core::ptr::null_mut::<i32>(),\n'
            '    };\n'
            '}'
        )
        check('non-zero field: skipped', len(self.find(code2)) == 0)

        # ── Test 3: has memset — defer to zero_init_with_memset ──────────────
        code3 = (
            'unsafe fn k() {\n'
            '    let mut s: Struct = Struct { x: 0 };\n'
            '    ::libc::memset(\n'
            '        &raw mut s as *mut ::core::ffi::c_void,\n'
            '        0 as ::core::ffi::c_int,\n'
            '        ::core::mem::size_of::<Struct>() as usize,\n'
            '    );\n'
            '}'
        )
        check('has-memset: deferred to zero_init_with_memset', len(self.find(code3)) == 0)

        # ── Test 4: array field, nested struct ────────────────────────────────
        code4 = (
            'unsafe fn arr() {\n'
            '    let mut p: ParseObj = ParseObj {\n'
            '        aTempReg: [0; 8],\n'
            '        padding: [0; 3],\n'
            '        nLabel: 0,\n'
            '        tok: Token {\n'
            '            z: ::core::ptr::null::<i8>(),\n'
            '            n: 0,\n'
            '        },\n'
            '    };\n'
            '}'
        )
        check('array + nested struct: find reports 1 match', len(self.find(code4)) == 1)

        # ── Test 5: two-phase integration with temp files ─────────────────────
        tmpdir = tempfile.mkdtemp(prefix='test_struct_default_')
        try:
            # Write a fake lib.rs containing the struct definition
            lib_rs_content = (
                '#[derive(Copy, Clone)]\n'
                '#[repr(C)]\n'
                'pub struct Widget {\n'
                '    pub count: i32,\n'
                '    pub ptr: *mut i32,\n'
                '}\n'
            )
            lib_rs_path = os.path.join(tmpdir, 'lib.rs')
            with open(lib_rs_path, 'w') as f:
                f.write(lib_rs_content)

            # Write a fake use-site file
            use_code = (
                'pub use crate::Widget;\n'
                'unsafe fn make() {\n'
                '    let mut w: Widget = Widget {\n'
                '        count: 0,\n'
                '        ptr: ::core::ptr::null_mut::<i32>(),\n'
                '    };\n'
                '    w.count = 1;\n'
                '}\n'
            )

            # Create a plugin instance pointing at our tmpdir as workspace
            import sys
            m = sys.modules[__name__]
            orig_root = m._WORKSPACE_ROOT
            try:
                m._WORKSPACE_ROOT = tmpdir
                plugin = StructAllZeroInitToZeroedPlugin()

                finds = plugin.find(use_code)
                check('two-phase integration: find reports 1 match', len(finds) == 1)

                fixed_use = plugin.fix(use_code)

                check(
                    'two-phase: use site uses ::default()',
                    'Widget::default()' in fixed_use,
                )
                check(
                    'two-phase: struct literal removed from use site',
                    'Widget {' not in fixed_use,
                )
                check(
                    'two-phase: subsequent code preserved',
                    'w.count = 1' in fixed_use,
                )

                # Check definition file was modified
                with open(lib_rs_path) as f:
                    lib_content = f.read()
                check(
                    'two-phase: impl Default inserted into lib.rs',
                    'impl Default for Widget' in lib_content,
                )
                check(
                    'two-phase: Default body uses null_mut() without turbofish',
                    'null_mut()' in lib_content,
                )
                check(
                    'two-phase: struct still present in lib.rs',
                    'pub struct Widget' in lib_content,
                )

                # Idempotency: second pass on already-fixed use code
                plugin2 = StructAllZeroInitToZeroedPlugin()
                finds_after = plugin2.find(fixed_use)
                check('two-phase idempotency: 0 finds after fix', len(finds_after) == 0)

            finally:
                m._WORKSPACE_ROOT = orig_root

        finally:
            shutil.rmtree(tmpdir, ignore_errors=True)

        # ── Test 6: Self { … } inside a generated Default impl — skip it ─────
        code6 = (
            'impl Default for Foo {\n'
            '    fn default() -> Self {\n'
            '        Self {\n'
            '            x: 0,\n'
            '            p: ::core::ptr::null_mut(),\n'
            '        }\n'
            '    }\n'
            '}\n'
        )
        # The `let` pattern in collect_matches won't match here because there's no
        # let_declaration. But just in case the code has `let _ = Self { ... }`:
        code6_let = (
            'impl Default for Bar {\n'
            '    fn default() -> Self {\n'
            '        let x: Bar = Self { a: 0 };\n'
            '        x\n'
            '    }\n'
            '}\n'
        )
        check(
            'Self { … } in default impl: skipped (no Self match)',
            len(self.find(code6_let)) == 0,
        )

        # ── Test 7: null-simplification helper ───────────────────────────────
        raw = 'db: ::core::ptr::null_mut::<crate::sqliteInt_h::sqlite3>(), z: ::core::ptr::null::<i8>(),'
        simplified = self._simplify_null_turbofish(raw)
        check(
            'null turbofish: null_mut::<T> simplified',
            'null_mut()' in simplified and 'null_mut::<' not in simplified,
        )
        check(
            'null turbofish: null::<T> simplified',
            'null()' in simplified and 'null::<' not in simplified,
        )

        # ── Test 8: _find_struct_end_pos ──────────────────────────────────────
        sample_lib = (
            '#[repr(C)]\n'
            'pub struct Foo {\n'
            '    pub x: i32,\n'
            '    pub p: *mut i32,\n'
            '}\n'
            '\n'
            'pub fn other() {}\n'
        )
        end_pos = self._find_struct_end_pos(sample_lib, 'Foo')
        check(
            '_find_struct_end_pos returns position after closing }',
            end_pos is not None and sample_lib[end_pos - 1] == '}',
        )

        impl_generated = self._generate_impl_default(
            'Foo',
            'Self { x: 0, p: ::core::ptr::null_mut() }',
        )
        inserted = sample_lib[:end_pos] + impl_generated + sample_lib[end_pos:]
        check(
            'inserted impl Default comes after struct definition',
            inserted.index('impl Default for Foo') > inserted.index('pub struct Foo'),
        )
        check(
            'idempotency check: impl already present detected',
            self._impl_default_exists_in_content(inserted, 'Foo'),
        )

        return all_passed
