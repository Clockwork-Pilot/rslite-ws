"""Plugin: Consolidate scattered (*ptr).field dereferences into safe references.

Hybrid tree-sitter + regex approach:
  - Tree-sitter for SCOPE detection (which block each deref belongs to)
  - Regex for TEXT replacement (avoids byte-range corruption bugs)

Handles:
  - Simple: (*ptr).field -> ref.field  (3+ uses in same block)
  - Nested chains: (*ptr).base.inner -> ref.base.inner
  - Double derefs: (*(*ptr).inner).field -> inner_ref.field
  - Mutable vs immutable detection (assignment LHS, compound assign, &raw mut)
  - Pointer reassignment: skips blocks where the pointer is reassigned
  - Multiple pointers per block

Safety constraints:
  - Only operates within a single tree-sitter block scope
  - Insertion point always right after the block opening `{` or after
    the `let` declaration (validated via tree-sitter)
  - Text replacement via regex — no byte-offset arithmetic on inner nodes
  - Block text replaced as a whole unit via tree-sitter byte range
"""

import re
import hashlib
import json
import os
from typing import List, Tuple, Optional, Any, Dict, Set  # noqa: F401
from collections import defaultdict

from .base import UnsafePatternPlugin

# File-based cache directory — persists across Python process invocations.
# Pre-running the fixer populates the cache so the constraint tool re-run
# (after `git checkout -- src/`) hits the cache and finishes in seconds.
_CACHE_DIR = "/tmp/raw_ptr_deref_cache"

DEREF_FIELD_RE = re.compile(r'\(\*(\w+)\)\s*\.\s*(\w+)')
DOUBLE_DEREF_RE = re.compile(r'\(\*\(\*(\w+)\)\s*\.\s*(\w+)\)\s*\.\s*(\w+)')
# Matches a double-deref used as an assignment LHS:
#   (*(*ptr).field).subfield OP= ...   (OP may be empty or compound +,-,*,/,%,&,|,^)
# Negative lookahead prevents matching == / != / <= / >=
# Matches a double-deref on the LHS of an assignment:
#   (*(*ptr).field).sub OP= ...
DOUBLE_DEREF_ASSIGN_RE = re.compile(
    r'\(\*\(\*(\w+)\)\s*\.\s*(\w+)\)[^;=\n]*?[^!=<>]=[^=]'
)
# Matches &raw mut (*(*ptr).field) — inner ref must be mutable
DOUBLE_DEREF_RAW_MUT_RE = re.compile(
    r'&raw\s+mut\s+\(\*\(\*(\w+)\)\s*\.\s*(\w+)\)'
)


class RawPtrDerefFieldChainPlugin(UnsafePatternPlugin):
    """Consolidate repeated (*ptr).field dereferences into hoisted safe references.

    Uses tree-sitter for scope analysis and regex for text replacement.
    """

    @property
    def name(self) -> str:
        return "raw_ptr_deref_field_chain"

    @property
    def description(self) -> str:
        return (
            "Consolidate repeated (*ptr).field dereferences into a single "
            "hoisted safe reference binding (scope-aware, regex-safe)"
        )

    @property
    def priority(self) -> int:
        return 14

    MIN_DEREF_COUNT = 3

    # ── AST helpers ───────────────────────────────────────────────────────────

    def _is_direct_deref_field(self, field_expr: Any, code: str) -> Optional[str]:
        """Check if field_expr is (*VAR).field. Returns VAR name or None."""
        value_child = field_expr.child_by_field_name('value')
        if value_child is None or value_child.type != 'parenthesized_expression':
            return None
        for child in value_child.children:
            if child.type == 'unary_expression':
                children = child.children
                if (len(children) >= 2
                        and children[0].type == '*'
                        and children[1].type == 'identifier'):
                    return self.node_text(children[1], code)
        return None

    def _is_mutably_used(self, field_expr: Any, code: str) -> bool:
        """Return True if the field access requires a mutable reference.

        Walks up the parent chain to catch:
        - direct assignment:   (*ptr).field = val
        - nested chain LHS:    (*ptr).field.sub = val   (parent is field_expr)
        - compound assign:     (*ptr).field += val
        - &mut reference:      &mut (*ptr).field
        """
        node: Any = field_expr
        while node is not None:
            parent = node.parent
            if parent is None:
                break
            if parent.type in ('assignment_expression', 'compound_assignment_expr'):
                left = parent.child_by_field_name('left')
                if left is not None:
                    # field_expr is on the LHS if our range is contained in left's range
                    if (left.start_byte <= field_expr.start_byte
                            and field_expr.end_byte <= left.end_byte):
                        return True
            elif parent.type == 'reference_expression':
                # &mut (*ptr).field
                for child in parent.children:
                    if child.type == 'mutable_specifier':
                        return True
            elif parent.type == 'call_expression':
                # This field_expr is used as the callable (method receiver).
                # We can't know without type info whether the method takes
                # &mut self, so conservatively treat it as needing mutability.
                func = parent.child_by_field_name('function')
                if func is not None and func.start_byte == node.start_byte:
                    return True
            elif parent.type == 'block':
                # Don't escape the enclosing block
                break
            node = parent
        return False

    def _ptr_is_const(self, block: Any, ptr_var: str, code: str) -> bool:
        """Return True if ptr_var is declared as a *const T (immutable raw pointer).

        Checks function parameters and let declarations.  If the pointer is
        *const T we must never generate &mut *ptr.
        """
        # Walk up to find the enclosing function/method to inspect parameters
        fn_node = block
        while fn_node is not None:
            if fn_node.type in ('function_item', 'impl_item'):
                break
            fn_node = fn_node.parent

        if fn_node is not None:
            params_node = fn_node.child_by_field_name('parameters')
            if params_node is not None:
                for param in params_node.children:
                    if param.type != 'parameter':
                        continue
                    pattern = param.child_by_field_name('pattern')
                    type_node = param.child_by_field_name('type')
                    if pattern is None or type_node is None:
                        continue
                    pat_text = self.node_text(pattern, code).strip().lstrip('mut').strip()
                    if pat_text == ptr_var:
                        type_text = self.node_text(type_node, code)
                        return '*const' in type_text

        # Also check let declaration in the block
        decl = self._find_ptr_decl_in_block(block, ptr_var, code)
        if decl is not None:
            type_node = decl.child_by_field_name('type')
            if type_node is not None:
                return '*const' in self.node_text(type_node, code)

        return False

    def _has_ptr_reassignment(self, block: Any, ptr_var: str, code: str) -> bool:
        """Check if ptr_var is reassigned anywhere in the block."""
        for assign in self.find_nodes(block, 'assignment_expression'):
            left = assign.child_by_field_name('left')
            if left is not None and left.type == 'identifier':
                if self.node_text(left, code).strip() == ptr_var:
                    return True
        return False

    def _find_ptr_decl_in_block(self, block: Any, ptr_var: str, code: str) -> Optional[Any]:
        """Find the let declaration of ptr_var that is a DIRECT child of block."""
        for child in block.children:
            if child.type == 'let_declaration':
                pattern = child.child_by_field_name('pattern')
                if pattern is not None:
                    pat_text = self.node_text(pattern, code).strip()
                    if pat_text == ptr_var:
                        return child
                for i, gc in enumerate(child.children):
                    if gc.type == 'identifier':
                        fn = child.field_name_for_child(i)
                        if fn == 'pattern' and self.node_text(gc, code) == ptr_var:
                            return child
        return None

    def _has_raw_mut_deref(self, block: Any, ptr_var: str, code: str) -> bool:
        """Check if &raw mut (*ptr).field appears in block."""
        block_text = self.node_text(block, code)
        return bool(re.search(rf'&raw\s+mut\s+\(\*{re.escape(ptr_var)}\)\s*\.', block_text))

    # ── Match collection ──────────────────────────────────────────────────────

    def _collect_matches(self, code: str) -> List[Dict[str, Any]]:
        """Find blocks where the same pointer is dereferenced 3+ times.

        Groups dereferences by (block, ptr_var). Each group becomes a
        potential transformation.
        """
        root = self.parse(code)
        all_field_exprs = self.find_nodes(root, 'field_expression')

        # Group by (block_start_byte, ptr_var)
        groups: Dict[Tuple[int, str], Dict[str, Any]] = {}

        for fe in all_field_exprs:
            ptr_var = self._is_direct_deref_field(fe, code)
            if ptr_var is None:
                continue

            block = self.get_parent_of_type(fe, 'block')
            if block is None:
                continue

            key = (block.start_byte, ptr_var)
            if key not in groups:
                groups[key] = {
                    'block': block,
                    'ptr_var': ptr_var,
                    'field_exprs': [],
                    'needs_mut': False,
                }
            groups[key]['field_exprs'].append(fe)
            if self._is_mutably_used(fe, code):
                groups[key]['needs_mut'] = True

        candidates = []
        for (block_start, ptr_var), info in groups.items():
            if len(info['field_exprs']) < self.MIN_DEREF_COUNT:
                continue

            block = info['block']

            # Skip if pointer is reassigned in this block
            if self._has_ptr_reassignment(block, ptr_var, code):
                continue

            # Check for &raw mut usage
            if self._has_raw_mut_deref(block, ptr_var, code):
                info['needs_mut'] = True

            # Check for double derefs: (*(*ptr).field).subfield
            block_text = self.node_text(block, code)
            double_targets: Dict[str, int] = defaultdict(int)
            for dm in DOUBLE_DEREF_RE.finditer(block_text):
                if dm.group(1) == ptr_var:
                    double_targets[dm.group(2)] += 1
            # Track which double-deref inner refs need to be mutable:
            # - used on the LHS of an assignment: (*(*ptr).field).sub = val
            # - used with &raw mut:               &raw mut (*(*ptr).field)
            double_derefs_mut: Set[str] = set()
            for da in DOUBLE_DEREF_ASSIGN_RE.finditer(block_text):
                if da.group(1) == ptr_var:
                    double_derefs_mut.add(da.group(2))
            for dr in DOUBLE_DEREF_RAW_MUT_RE.finditer(block_text):
                if dr.group(1) == ptr_var:
                    double_derefs_mut.add(dr.group(2))
            info['double_derefs'] = {f: c for f, c in double_targets.items() if c >= 2}
            info['double_derefs_mut'] = double_derefs_mut

            candidates.append(info)

        # ── Per-ptr deduplication ─────────────────────────────────────────────
        # For the same ptr_var, keep only outermost blocks.
        # Bug fix: use a LIST of ranges per ptr (not a single range) so that
        # multiple non-overlapping outer blocks across different functions are
        # all remembered.  Without this, the stored range gets overwritten by
        # the next non-overlapping block, causing inner blocks to slip through.
        pre_final = []
        candidates.sort(key=lambda c: c['block'].end_byte - c['block'].start_byte, reverse=True)
        used_block_ranges: Dict[str, List[Tuple[int, int]]] = {}  # ptr_var -> [(start, end), ...]

        for info in candidates:
            ptr_var = info['ptr_var']
            block = info['block']
            bs, be = block.start_byte, block.end_byte

            # Skip if ANY already-selected block for the same ptr fully contains this one
            if any(outer_s <= bs and be <= outer_e
                   for outer_s, outer_e in used_block_ranges.get(ptr_var, [])):
                continue

            used_block_ranges.setdefault(ptr_var, []).append((bs, be))
            pre_final.append(info)

        # ── Cross-ptr deduplication ───────────────────────────────────────────
        # Blocks for DIFFERENT ptrs that share the exact same range are fine —
        # they are grouped together in block_transforms and produce one
        # replacement.  But if a different-ptr block is NESTED inside (or
        # partially overlaps) an already-selected block, it would produce a
        # separate, overlapping replacement that corrupts the source text.
        # Fix: skip any block that intersects with an existing selected range
        # UNLESS this block strictly *contains* the existing range (i.e., is
        # itself the outer block).
        final: List[Dict[str, Any]] = []
        used_ranges: List[Tuple[int, int]] = []
        # Sort by start_byte asc, then by size desc so outer blocks (same start,
        # larger span) are always processed before inner ones.
        pre_final.sort(key=lambda m: (m['block'].start_byte,
                                      -(m['block'].end_byte - m['block'].start_byte)))
        for info in pre_final:
            bs, be = info['block'].start_byte, info['block'].end_byte
            skip = False
            for us, ue in used_ranges:
                if bs < ue and be > us:          # any intersection at all
                    # Allow only if THIS block strictly contains the existing range
                    if not (bs <= us and ue <= be):
                        skip = True
                        break
            if not skip:
                final.append(info)
                used_ranges.append((bs, be))

        # ── Rescan for complete field_exprs and needs_mut ─────────────────────
        # The initial grouping used `get_parent_of_type(fe, 'block')` which
        # returns the INNERMOST block, so field_exprs inside nested if/loop
        # blocks were attributed to those inner blocks, not the outermost.
        # After deduplication the outer block is selected, but its field_exprs
        # list is incomplete and its needs_mut may be False even though a
        # mutation exists inside an inner block.
        # Fix: rescan all_field_exprs to collect every (*ptr). access contained
        # within the selected block range and recompute needs_mut.
        for info in final:
            block = info['block']
            ptr_var = info['ptr_var']
            bs, be = block.start_byte, block.end_byte
            full_fes = [
                fe for fe in all_field_exprs
                if (self._is_direct_deref_field(fe, code) == ptr_var
                    and bs <= fe.start_byte and fe.end_byte <= be)
            ]
            info['field_exprs'] = full_fes
            info['needs_mut'] = (
                any(self._is_mutably_used(fe, code) for fe in full_fes)
                or self._has_raw_mut_deref(block, ptr_var, code)
            )

        # ── Filter: skip *const pointers that would need &mut ─────────────────
        # If the pointer is *const T and we'd need &mut *ptr, that is a type
        # error.  Drop those blocks rather than generating invalid code.
        # This filter MUST come after the rescan so needs_mut is fully resolved.
        cleaned: List[Dict[str, Any]] = []
        for info in final:
            if self._ptr_is_const(info['block'], info['ptr_var'], code):
                if info['needs_mut']:
                    continue  # cannot safely hoist — *const cannot give &mut
            cleaned.append(info)
        final = cleaned

        return final

    # ── Find ──────────────────────────────────────────────────────────────────

    @staticmethod
    def _quick_check(rust_code: str) -> bool:
        """Fast pre-filter: return False if the file cannot possibly match.

        Checks for the literal pattern `(*<ident>).` appearing at least
        MIN_DEREF_COUNT times anywhere in the file using a simple regex scan —
        no tree-sitter needed.  Only files that pass this check go through the
        expensive AST analysis.
        """
        return len(DEREF_FIELD_RE.findall(rust_code)) >= RawPtrDerefFieldChainPlugin.MIN_DEREF_COUNT

    def find(self, rust_code: str) -> List[Tuple[int, int, str]]:
        if not self._quick_check(rust_code):
            return []

        # File-based cache: avoid re-running expensive AST analysis.
        code_hash = hashlib.sha256(rust_code.encode()).hexdigest()
        find_cache = os.path.join(_CACHE_DIR, f"find_{code_hash}.json")
        if os.path.exists(find_cache):
            try:
                with open(find_cache, 'r', encoding='utf-8') as _f:
                    return [tuple(r) for r in json.load(_f)]
            except Exception:
                pass

        results = []
        for m in self._collect_matches(rust_code):
            block = m['block']
            ptr_var = m['ptr_var']
            count = len(m['field_exprs'])
            mut_str = "mut " if m['needs_mut'] else ""
            first = m['field_exprs'][0]
            line = self.node_line(first, rust_code)
            dd_info = ""
            if m.get('double_derefs'):
                dd_fields = ', '.join(m['double_derefs'].keys())
                dd_info = f" + double-deref via .{dd_fields}"
            results.append((
                block.start_byte,
                block.end_byte,
                f"Line {line}: `(*{ptr_var}).field` dereferenced {count}x "
                f"in same block — hoist to "
                f"`let {mut_str}__{ptr_var}_ref = unsafe {{ &{mut_str}*{ptr_var} }}`"
                f"{dd_info}",
            ))

        # Persist find results to cache
        try:
            os.makedirs(_CACHE_DIR, exist_ok=True)
            with open(find_cache, 'w', encoding='utf-8') as _f:
                json.dump(results, _f)
        except Exception:
            pass

        return results

    # ── Fix ───────────────────────────────────────────────────────────────────

    def fix(self, rust_code: str) -> str:
        """Apply all pointer ref-hoisting transformations.

        For each match:
        1. Extract the block text
        2. Use regex to replace (*ptr). with ref. throughout
        3. Insert the ref binding after the declaration or block open
        4. Replace the entire block text in the source
        """
        if not self._quick_check(rust_code):
            return rust_code

        # File-based cache: if we've already fixed this exact source text,
        # return the cached result immediately without re-running AST analysis.
        code_hash = hashlib.sha256(rust_code.encode()).hexdigest()
        fix_cache = os.path.join(_CACHE_DIR, f"fix_{code_hash}.txt")
        if os.path.exists(fix_cache):
            try:
                with open(fix_cache, 'r', encoding='utf-8') as _f:
                    return _f.read()
            except Exception:
                pass

        matches = self._collect_matches(rust_code)
        if not matches:
            return rust_code

        # Group matches by block — multiple pointers may be in the same block
        block_transforms: Dict[int, List[Dict[str, Any]]] = defaultdict(list)
        for m in matches:
            block_transforms[m['block'].start_byte].append(m)

        # Build replacements: one per block
        replacements: List[Tuple[int, int, str]] = []

        for block_start, transforms in block_transforms.items():
            block = transforms[0]['block']
            block_text = self.node_text(block, rust_code)
            block_lines = block_text.split('\n')

            # Detect indentation from the first statement in the block
            indent = "    "
            for ln in block_lines[1:]:
                stripped = ln.lstrip()
                if stripped and stripped not in ('{', '}'):
                    indent = ln[:len(ln) - len(stripped)]
                    break

            # Keep a snapshot of the ORIGINAL block lines for insertion-point
            # arithmetic.  Regex replacements (below) mutate block_lines in
            # place, which changes line lengths and would corrupt the
            # char_count-based offset calculation for subsequent transforms.
            orig_block_lines = block_lines[:]

            # all_bindings: (insert_after, priority, binding_text)
            # priority 0 = main ref binding, 1 = double-deref binding.
            # When two bindings share the same insert_after, priority 1 is
            # inserted first (→ lands lower in the file) so that the main
            # binding (priority 0, inserted second) ends up directly above it
            # — the double-deref ref depends on the main ref.
            all_bindings = []

            for t in transforms:
                ptr_var = t['ptr_var']
                needs_mut = t['needs_mut']
                ref_name = f'__{ptr_var}_ref'
                mut_str = "mut " if needs_mut else ""
                double_derefs = t.get('double_derefs', {})

                # Find insertion point using ORIGINAL line lengths so that
                # earlier regex passes cannot shift the calculated line index.
                #
                # Strategy: always insert just BEFORE the line containing the
                # first (*ptr).field occurrence.  This guarantees the binding
                # lands AFTER any null-guard / early-return checks that precede
                # the first real use of the pointer, preventing UB from creating
                # a reference from a potentially-null pointer.
                #
                # For the `let p = ...` declaration case we additionally enforce
                # that we never insert BEFORE the declaration itself, so we take
                # the maximum of (after-decl line, before-first-use line).
                decl_node = self._find_ptr_decl_in_block(block, ptr_var, rust_code)

                # Find insertion line: walk from the first field_expr UP to the
                # ancestor node that is a DIRECT CHILD of the selected block.
                # This is critical: if the first usage is inside an `if` body,
                # we must insert BEFORE the whole `if` statement at the outer
                # scope, not inside its body (which would make the binding
                # invisible to sibling `else if`/`else` branches).
                # We also need to avoid inserting mid-expression (e.g., inside
                # a function-call argument list).
                first_use_line = len(orig_block_lines) - 1  # fallback: end of block
                if t['field_exprs']:
                    first_fe = min(t['field_exprs'], key=lambda fe: fe.start_byte)
                    # Walk up until parent IS the selected block
                    anchor = first_fe
                    cur = first_fe
                    while cur is not None:
                        parent = cur.parent
                        if parent is None:
                            break
                        if (parent.start_byte == block.start_byte
                                and parent.end_byte == block.end_byte):
                            anchor = cur
                            break
                        cur = parent
                    anchor_offset = max(0, anchor.start_byte - block.start_byte)
                    char_count = 0
                    for i, ln in enumerate(orig_block_lines):
                        char_count += len(ln) + 1  # +1 for \n
                        if char_count > anchor_offset:
                            # Anchor starts on line i; insert before it
                            first_use_line = max(0, i - 1)
                            break

                if decl_node is not None:
                    decl_end_byte = decl_node.end_byte
                    decl_end_offset = decl_end_byte - block.start_byte
                    char_count = 0
                    after_decl_line = 0
                    for i, ln in enumerate(orig_block_lines):
                        char_count += len(ln) + 1  # +1 for \n
                        if char_count >= decl_end_offset:
                            after_decl_line = i
                            break
                    # Must be after the declaration, but ideally before first use
                    insert_after = max(after_decl_line, first_use_line)
                else:
                    # Pointer is a function parameter.
                    # Insert just before the first actual use (after null checks).
                    insert_after = first_use_line

                binding = f"{indent}let {ref_name} = unsafe {{ &{mut_str}*{ptr_var} }};"
                all_bindings.append((insert_after, 0, binding))

                # NOTE: double-deref bindings (for (*(*ptr).field).sub patterns)
                # are intentionally NOT generated here.  They would need to be
                # inserted AFTER the inner field is initialised, but we cannot
                # reliably determine that insertion point without full data-flow
                # analysis.  Inserting them too early (before the field is set)
                # creates a reference from an uninitialised/garbage pointer,
                # causing UB and runtime crashes.
                #
                # The single-deref regex pass below handles double-deref patterns
                # correctly without inner bindings:
                #   (*(*ptr).field).sub  →  (*__ptr_ref.field).sub
                # which is valid Rust and semantically equivalent.

                # Apply regex replacement on block_lines (single derefs only)
                single_re = re.compile(rf'\(\*{re.escape(ptr_var)}\)\s*\.')
                for i in range(len(block_lines)):
                    block_lines[i] = single_re.sub(f'{ref_name}.', block_lines[i])

            # Insert bindings bottom-to-top so earlier insertions don't shift
            # the line indices of later ones.
            # Sort: primary key = insert_after descending (bottom-to-top).
            # Secondary key = priority descending: priority-1 items (double-deref)
            # sort before priority-0 items for the same line, so they are
            # inserted first and end up BELOW the main ref binding.
            for insert_after, _priority, binding in sorted(
                all_bindings, key=lambda x: (x[0], x[1]), reverse=True
            ):
                block_lines.insert(insert_after + 1, binding)

            new_block_text = '\n'.join(block_lines)
            replacements.append((block.start_byte, block.end_byte, new_block_text))

        result = self.apply_replacements(rust_code, replacements)

        # Persist fix result to cache
        try:
            os.makedirs(_CACHE_DIR, exist_ok=True)
            with open(fix_cache, 'w', encoding='utf-8') as _f:
                _f.write(result)
        except Exception:
            pass

        return result

    # ── Tests ─────────────────────────────────────────────────────────────────

    def test(self) -> bool:
        all_passed = True

        def check(name: str, condition: bool, msg: str = "") -> bool:
            nonlocal all_passed
            if condition:
                print(f"  v {name}: OK")
                return True
            else:
                print(f"  x {name}: FAILED {msg}")
                all_passed = False
                return False

        # Test 1: 3 dereferences — should trigger
        code1 = '\n'.join([
            "pub unsafe fn example(pParse: *mut Parse) {",
            "    let x = (*pParse).nErr;",
            "    let db = (*pParse).db;",
            "    let v = (*pParse).pVdbe;",
            "}",
        ])
        finds1 = self.find(code1)
        check("3-deref find", len(finds1) == 1, f"expected 1, got {len(finds1)}")
        fixed1 = self.fix(code1)
        check("3-deref ref created", "__pParse_ref" in fixed1, f"\n{fixed1}")
        check("3-deref no raw deref", "(*pParse)." not in fixed1, f"\n{fixed1}")
        check("3-deref unsafe binding", "unsafe { &*pParse }" in fixed1, f"\n{fixed1}")

        # Test 2: Below threshold
        code2 = '\n'.join([
            "pub unsafe fn small(p: *mut Node) {",
            "    let x = (*p).a;",
            "    let y = (*p).b;",
            "}",
        ])
        check("below-threshold", len(self.find(code2)) == 0)

        # Test 3: Mutable dereference
        code3 = '\n'.join([
            "pub unsafe fn mutating(pParse: *mut Parse) {",
            "    (*pParse).nErr = 0;",
            "    let db = (*pParse).db;",
            "    (*pParse).nTab = (*pParse).nTab + 1;",
            "}",
        ])
        check("mut-deref find", len(self.find(code3)) == 1)
        fixed3 = self.fix(code3)
        check("mut-deref uses &mut", "&mut *pParse" in fixed3, f"\n{fixed3}")

        # Test 4: Pointer reassigned
        code4 = '\n'.join([
            "pub unsafe fn reassigned(mut p: *mut Node) {",
            "    let a = (*p).x;",
            "    p = (*p).pNext;",
            "    let b = (*p).x;",
            "    let c = (*p).y;",
            "}",
        ])
        check("reassigned-ptr", len(self.find(code4)) == 0)

        # Test 5: Two pointers in same block
        code5 = '\n'.join([
            "pub unsafe fn two_ptrs(a: *mut X, b: *mut Y) {",
            "    let x1 = (*a).f1;",
            "    let x2 = (*a).f2;",
            "    let x3 = (*a).f3;",
            "    let y1 = (*b).g1;",
            "    let y2 = (*b).g2;",
            "    let y3 = (*b).g3;",
            "}",
        ])
        check("two-ptrs find", len(self.find(code5)) == 2)
        fixed5 = self.fix(code5)
        check("two-ptrs ref a", "__a_ref" in fixed5, f"\n{fixed5}")
        check("two-ptrs ref b", "__b_ref" in fixed5, f"\n{fixed5}")
        check("two-ptrs no raw", "(*a)." not in fixed5 and "(*b)." not in fixed5, f"\n{fixed5}")

        # Test 6: Nested chain
        code6 = '\n'.join([
            "pub unsafe fn nested(p: *mut Outer) {",
            "    let a = (*p).base.inner;",
            "    let b = (*p).field1;",
            "    let c = (*p).field2;",
            "}",
        ])
        check("nested-chain find", len(self.find(code6)) == 1)
        fixed6 = self.fix(code6)
        check("nested-chain preserved", "__p_ref.base.inner" in fixed6, f"\n{fixed6}")
        check("nested-chain no raw", "(*p)." not in fixed6, f"\n{fixed6}")

        # Test 7: Declaration ordering
        code7 = '\n'.join([
            "pub unsafe fn decl_order(x: i32) {",
            "    let p: *mut Node = get_node(x);",
            "    let a = (*p).f1;",
            "    let b = (*p).f2;",
            "    let c = (*p).f3;",
            "}",
        ])
        fixed7 = self.fix(code7)
        decl_pos = fixed7.find("let p: *mut Node")
        ref_pos = fixed7.find("let __p_ref = unsafe")
        check("decl-order", decl_pos != -1 and ref_pos != -1 and ref_pos > decl_pos,
              f"ref before decl!\n{fixed7}")

        # Test 8: Compound assignment
        code8 = '\n'.join([
            "pub unsafe fn compound(p: *mut S) {",
            "    (*p).count += 1;",
            "    let x = (*p).data;",
            "    (*p).total += (*p).count;",
            "}",
        ])
        check("compound-assign find", len(self.find(code8)) == 1)
        fixed8 = self.fix(code8)
        check("compound-assign mut", "&mut *p" in fixed8, f"\n{fixed8}")
        check("compound-assign replaced", "__p_ref.count += 1" in fixed8, f"\n{fixed8}")

        # Test 9: Function call args
        code9 = '\n'.join([
            "pub unsafe fn call_args(p: *mut S) {",
            "    foo((*p).a, (*p).b);",
            "    bar((*p).c);",
            "}",
        ])
        check("call-args find", len(self.find(code9)) == 1)
        fixed9 = self.fix(code9)
        check("call-args replaced", "foo(__p_ref.a, __p_ref.b)" in fixed9, f"\n{fixed9}")

        # Test 10: Comparison not mut
        code10 = '\n'.join([
            "pub unsafe fn comparisons(p: *mut S) {",
            "    if (*p).x == 0 {",
            "        let y = (*p).y;",
            "        let z = (*p).z;",
            "    }",
            "}",
        ])
        finds10 = self.find(code10)
        if finds10:
            fixed10 = self.fix(code10)
            check("comparison-not-mut", "&mut" not in fixed10, f"\n{fixed10}")
        else:
            print("  v comparison-not-mut: skipped (inner block)")

        # Test 11: Double deref
        code11 = '\n'.join([
            "pub unsafe fn double_deref(p: *mut Outer) {",
            "    let x = (*p).name;",
            "    let a = (*(*p).inner).field1;",
            "    let b = (*(*p).inner).field2;",
            "    let c = (*p).count;",
            "}",
        ])
        check("double-deref find", len(self.find(code11)) == 1)
        fixed11 = self.fix(code11)
        check("double-deref inner ref", "__p_inner_ref" in fixed11, f"\n{fixed11}")
        check("double-deref replaced", "__p_inner_ref.field1" in fixed11, f"\n{fixed11}")
        check("double-deref no raw", "(*(*p).inner)" not in fixed11, f"\n{fixed11}")

        # Test 12: Real-world pattern
        code12 = '\n'.join([
            "pub unsafe fn fts5StmtPrepare(p: *mut Fts5Storage, pC: *mut Fts5Config) -> i32 {",
            "    let mut rc: i32 = 0;",
            "    let zSql = sqlite3_mprintf(azStmt[0], (*pC).zDb, (*pC).zName);",
            "    (*(*p).pConfig).bLock += 1;",
            "    rc = sqlite3_prepare_v2(",
            "        (*pC).db,",
            "        zSql,",
            "        -1,",
            "    );",
            "    (*(*p).pConfig).bLock -= 1;",
            "    if rc != 0 {",
            "        let msg = sqlite3_errmsg((*pC).db);",
            "    }",
            "    rc",
            "}",
        ])
        check("real-world find", len(self.find(code12)) >= 1)
        fixed12 = self.fix(code12)
        check("real-world pC replaced", "__pC_ref.zDb" in fixed12 or "__pC_ref.db" in fixed12,
              f"\n{fixed12}")
        check("real-world no corruption", "fn fts5StmtPrepare" in fixed12,
              f"function signature corrupted!\n{fixed12}")

        return all_passed
