"""Plugin: Replace verbose zero struct-init + memset with core::mem::zeroed().

C-to-Rust transpilers (like c2rust) emit a two-step pattern to zero-initialize
a local struct or array:

  Step 1 — satisfy Rust's "must initialize" requirement with a verbose literal:
      let mut sNC: NameContext = NameContext {
          pParse: ::core::ptr::null_mut::<Parse>(),
          pSrcList: ::core::ptr::null_mut::<SrcList>(),
          ...
      };

  Step 2 — zero out the memory the way the original C did it:
      ::libc::memset(
          &raw mut sNC as *mut ::core::ffi::c_void,
          0 as ::core::ffi::c_int,
          ::core::mem::size_of::<NameContext>() as size_t,
      );

Both steps together are semantically equivalent to:
      let mut sNC: NameContext = unsafe { ::core::mem::zeroed() };

This plugin detects the pair and collapses them into the idiomatic Rust form.

Engine features used (added to base.py):
  - is_zero_value_node()  — recognise 0 through chains of casts
  - get_statement_node()  — climb to the enclosing expression_statement
  - apply_replacements()  — batch multi-location edits in reverse byte order
"""

import re
from typing import List, Optional, Tuple, Any

from .base import UnsafePatternPlugin


class ZeroInitWithMemsetPlugin(UnsafePatternPlugin):
    """Collapse verbose zero-init + memset into core::mem::zeroed().

    Detects:
      let mut VAR: TYPE = TYPE { <all-zero fields> };
      ...
      ::libc::memset(&raw mut VAR as *mut c_void, 0, size_of::<TYPE>());

    And rewrites to:
      let mut VAR: TYPE = unsafe { ::core::mem::zeroed() };
    (removing the memset statement entirely)
    """

    @property
    def name(self) -> str:
        return "zero_init_with_memset"

    @property
    def description(self) -> str:
        return (
            "Replace verbose zero struct/array init + memset with "
            "unsafe { ::core::mem::zeroed() }"
        )

    @property
    def priority(self) -> int:
        return 10  # Highest — big structural transformation, reduces significant noise

    # ── variable-name extraction ──────────────────────────────────────────────

    def _extract_var_name(self, let_node: Any, code: str) -> Optional[str]:
        """Return the variable name bound by a let_declaration.

        In this tree-sitter-rust grammar, `let mut x: T = v;` is parsed as:
          let_declaration
            mutable_specifier   ("mut")
            pattern: identifier ("x")
            type: ...
            value: ...

        The `mutable_specifier` is a sibling of `pattern`, NOT wrapped inside it.
        """
        pattern = let_node.child_by_field_name('pattern')
        if pattern is not None and pattern.type == 'identifier':
            return self.node_text(pattern, code)

        # Fallback: walk children for the identifier that comes after
        # the optional mutable_specifier (but is still named "pattern")
        for i, child in enumerate(let_node.children):
            if child.type == 'identifier':
                fn = let_node.field_name_for_child(i)
                if fn == 'pattern':
                    return self.node_text(child, code)

        return None

    # ── initializer classification ────────────────────────────────────────────

    def _is_verbose_zero_init(self, value_node: Any) -> bool:
        """Return True if value_node is the kind of verbose literal we want to replace.

        We accept:
          struct_expression  — Foo { field: null_mut(), other: 0, ... }
          array_expression   — [Foo { ... }; N] where the element is a struct

        We REJECT simple already-idiomatic forms like [0; 5] or [null_mut(); 6].
        The C-transpiler verbose pattern always has a struct_expression as the
        array element; simple scalar/call repeats are already correct Rust.
        """
        if value_node is None:
            return False
        if value_node.type == 'struct_expression':
            return True
        if value_node.type == 'array_expression':
            # Only transform if the element is itself a struct literal.
            return any(c.type == 'struct_expression' for c in value_node.children)
        return False

    # ── memset detection ──────────────────────────────────────────────────────

    def _find_memset_zeroing(
        self, block_node: Any, var_name: str, code: str
    ) -> Optional[Any]:
        """Find a memset call inside block_node that zeroes var_name.

        Criteria:
          1. Function path ends with "memset"
          2. First argument contains  &raw mut VAR_NAME  (possibly then cast)
          3. Second argument evaluates to integer 0 (possibly cast)
          4. Third argument contains "size_of"
          5. The memset must be a direct statement of block_node, NOT nested
             inside a conditional or loop sub-block.  A memset inside
             `if (flag) { memset(...) }` is a runtime conditional reset, not
             an initialization pair — removing it would change semantics.
        """
        for call in self.find_nodes(block_node, 'call_expression'):
            # Guard: the call's immediate parent block must be the same block
            # as block_node (compare by start position to avoid identity issues).
            call_block = self.get_parent_of_type(call, 'block')
            if call_block is None or call_block.start_byte != block_node.start_byte:
                continue
            call_text = self.node_text(call, code)

            # Fast pre-filter — both tokens must appear in the call text
            if 'memset' not in call_text or var_name not in call_text:
                continue

            fn_node = call.child_by_field_name('function')
            if fn_node is None:
                continue
            fn_text = self.node_text(fn_node, code)
            if not (fn_text == 'memset' or fn_text.endswith('::memset')):
                continue

            args_node = call.child_by_field_name('arguments')
            if args_node is None:
                continue

            arg_nodes = [c for c in args_node.children
                         if c.type not in ('(', ')', ',', 'comment')]
            if len(arg_nodes) != 3:
                continue

            # Arg 0: must contain  &raw mut VAR_NAME
            first_text = self.node_text(arg_nodes[0], code)
            if not re.search(
                rf'&\s*raw\s+mut\s+{re.escape(var_name)}\b', first_text
            ):
                continue

            # Arg 1: must be the integer 0 (possibly cast)
            if not self.is_zero_value_node(arg_nodes[1], code):
                continue

            # Arg 2: must mention size_of
            if 'size_of' not in self.node_text(arg_nodes[2], code):
                continue

            return call

        return None

    # ── statement removal range ───────────────────────────────────────────────

    def _statement_removal_range(self, code: str, stmt_node: Any) -> Tuple[int, int]:
        """Return the byte range [start, end) to delete a statement cleanly.

        Extends leftward to eat the line's leading indentation, and rightward
        to eat the trailing newline, so no blank line is left behind.

        Example — removing the middle line:
            Before:  ...field = x;\\n    memset(...);\\ n    next = y;\\n
            After:   ...field = x;\\n    next = y;\\n
        """
        start = stmt_node.start_byte
        end = stmt_node.end_byte

        # Eat trailing newline
        if end < len(code) and code[end] == '\n':
            end += 1

            # If everything between the previous newline and `start` is
            # whitespace (indentation only), eat that too so we don't leave
            # an empty line.
            prev_nl = code.rfind('\n', 0, start)
            line_start = (prev_nl + 1) if prev_nl != -1 else 0
            if code[line_start:start].strip() == '':
                start = line_start

        return start, end

    # ── match collection ──────────────────────────────────────────────────────

    def _collect_matches(self, code: str):
        """Return list of (let_node, value_node, memset_call) triples."""
        root = self.parse(code)
        matches = []

        for let_node in self.find_nodes(root, 'let_declaration'):
            var_name = self._extract_var_name(let_node, code)
            if var_name is None:
                continue

            value_node = let_node.child_by_field_name('value')
            if not self._is_verbose_zero_init(value_node):
                continue

            # Look for the corresponding memset in the same block scope
            block = self.get_parent_of_type(let_node, 'block')
            if block is None:
                continue

            memset_call = self._find_memset_zeroing(block, var_name, code)
            if memset_call is not None:
                matches.append((let_node, value_node, memset_call, var_name))

        return matches

    # ── interface ─────────────────────────────────────────────────────────────

    def find(self, rust_code: str) -> List[Tuple[int, int, str]]:
        """Return findings for each (verbose init, memset) pair detected."""
        results = []
        for let_node, value_node, _memset_call, var_name in self._collect_matches(
            rust_code
        ):
            line = self.node_line(let_node, rust_code)
            init_preview = self.node_text(value_node, rust_code)[:50].replace('\n', ' ')
            results.append((
                let_node.start_byte,
                let_node.end_byte,
                f"Line {line}: verbose zero-init of '{var_name}' "
                f"followed by memset (init: {init_preview}...)",
            ))
        return results

    def fix(self, rust_code: str) -> str:
        """Apply the transformation: collapse verbose init + memset into zeroed().

        Two edits per match (applied in reverse byte order via apply_replacements):
          1. Replace the initializer expression with  unsafe { ::core::mem::zeroed() }
          2. Delete the entire memset statement (including its indentation line)
        """
        matches = self._collect_matches(rust_code)
        if not matches:
            return rust_code

        replacements: List[Tuple[int, int, str]] = []

        for _let_node, value_node, memset_call, _var_name in matches:
            # Edit 1: replace the verbose initializer expression
            replacements.append((
                value_node.start_byte,
                value_node.end_byte,
                "unsafe { ::core::mem::zeroed() }",
            ))

            # Edit 2: delete the memset expression_statement
            stmt = self.get_statement_node(memset_call)
            if stmt is not None:
                rm_start, rm_end = self._statement_removal_range(rust_code, stmt)
                replacements.append((rm_start, rm_end, ""))

        return self.apply_replacements(rust_code, replacements)

    # ── tests ─────────────────────────────────────────────────────────────────

    def test(self) -> bool:
        """Self-contained tests for zero_init_with_memset."""
        all_passed = True

        # ── test 1: basic struct + immediate memset ───────────────────────────
        code1 = (
            "fn f(db: *mut Db) {\n"
            "    let mut sNC: NameCtx = NameCtx {\n"
            "        pParse: ::core::ptr::null_mut::<Parse>(),\n"
            "        nRef: 0,\n"
            "    };\n"
            "    ::libc::memset(\n"
            "        &raw mut sNC as *mut ::core::ffi::c_void,\n"
            "        0 as ::core::ffi::c_int,\n"
            "        ::core::mem::size_of::<NameCtx>() as usize,\n"
            "    );\n"
            "    sNC.pParse = db as *mut Parse;\n"
            "}"
        )
        finds1 = self.find(code1)
        if len(finds1) != 1:
            print(f"  ✗ basic struct+memset: expected 1 find, got {len(finds1)}")
            all_passed = False
        else:
            print("  ✓ basic struct+memset: find OK")

        fixed1 = self.fix(code1)
        if "unsafe { ::core::mem::zeroed() }" not in fixed1:
            print("  ✗ basic struct+memset: zeroed() not in output")
            print(f"     output: {fixed1!r}")
            all_passed = False
        elif "memset" in fixed1:
            print("  ✗ basic struct+memset: memset still present after fix")
            all_passed = False
        elif "NameCtx {" in fixed1:
            print("  ✗ basic struct+memset: verbose struct literal still present")
            all_passed = False
        else:
            print("  ✓ basic struct+memset: fix OK")

        # ── test 2: memset with cast-chain zeros ──────────────────────────────
        code2 = (
            "fn g() {\n"
            "    let mut buf: MyBuf = MyBuf { x: 0 };\n"
            "    ::libc::memset(\n"
            "        &raw mut buf as *mut ::core::ffi::c_void,\n"
            "        0 as ::core::ffi::c_int,\n"
            "        ::core::mem::size_of::<MyBuf>() as u64,\n"
            "    );\n"
            "}"
        )
        fixed2 = self.fix(code2)
        if "unsafe { ::core::mem::zeroed() }" not in fixed2:
            print("  ✗ cast-chain zeros: zeroed() not in output")
            all_passed = False
        elif "memset" in fixed2:
            print("  ✗ cast-chain zeros: memset still present")
            all_passed = False
        else:
            print("  ✓ cast-chain zeros: fix OK")

        # ── test 3: no memset — should not transform ───────────────────────────
        code3 = (
            "fn h() {\n"
            "    let mut x: Foo = Foo { a: 0, b: ::core::ptr::null_mut() };\n"
            "    x.a = 1;\n"
            "}"
        )
        finds3 = self.find(code3)
        if finds3:
            print(f"  ✗ no-memset case: expected 0 finds, got {len(finds3)}")
            all_passed = False
        else:
            print("  ✓ no-memset case: correctly skipped")

        # ── test 4: memset non-zero value — should not transform ───────────────
        code4 = (
            "fn k() {\n"
            "    let mut arr: [u8; 8] = [0u8; 8];\n"
            "    ::libc::memset(\n"
            "        &raw mut arr as *mut ::core::ffi::c_void,\n"
            "        0xFF as ::core::ffi::c_int,\n"
            "        ::core::mem::size_of::<[u8; 8]>() as usize,\n"
            "    );\n"
            "}"
        )
        finds4 = self.find(code4)
        if finds4:
            print(f"  ✗ non-zero memset: expected 0 finds, got {len(finds4)}")
            all_passed = False
        else:
            print("  ✓ non-zero memset: correctly skipped")

        # ── test 5: post-fix still has subsequent code ─────────────────────────
        code5 = (
            "fn m(p: *mut Parse) {\n"
            "    let mut ctx: Ctx = Ctx { field: ::core::ptr::null_mut() };\n"
            "    ::libc::memset(\n"
            "        &raw mut ctx as *mut ::core::ffi::c_void,\n"
            "        0 as ::core::ffi::c_int,\n"
            "        ::core::mem::size_of::<Ctx>() as usize,\n"
            "    );\n"
            "    ctx.field = p;\n"
            "    use_ctx(&raw mut ctx);\n"
            "}"
        )
        fixed5 = self.fix(code5)
        if "ctx.field = p;" not in fixed5:
            print("  ✗ subsequent code: code after memset was lost")
            print(f"     output: {fixed5!r}")
            all_passed = False
        elif "use_ctx(&raw mut ctx);" not in fixed5:
            print("  ✗ subsequent code: use_ctx call was lost")
            all_passed = False
        elif "memset" in fixed5:
            print("  ✗ subsequent code: memset still present")
            all_passed = False
        else:
            print("  ✓ subsequent code: surrounding code preserved")

        return all_passed
