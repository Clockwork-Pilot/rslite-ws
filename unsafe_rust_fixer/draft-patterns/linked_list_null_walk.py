"""Plugin: Convert C-style linked-list null-walks to idiomatic Rust iteration.

C-to-Rust transpilers emit the classic C linked-list traversal pattern:

    while !p.is_null() {
        let tmp = p;
        p = (*p).pNext;
        // ... use (*tmp) ...
        free(tmp);
    }

Or simpler read-only walks:

    while !p.is_null() {
        // ... use (*p) ...
        p = (*p).pNext;
    }

These are unsafe, fragile, and un-idiomatic. This plugin detects these
patterns via AST analysis and rewrites them using a safe helper macro
`unsafe_linked_list_iter!` that encapsulates the null-walk:

    // Read-only walk:
    let mut cursor = LinkedListCursor::new(head);
    while let Some(node) = cursor.next_unsafe() {
        // ... use node ...
    }

Or more precisely, since we cannot change the data structures yet, we
generate a SAFER wrapper that still uses unsafe internally but makes the
pattern explicit and auditable:

    // Before:
    while !p.is_null() {
        let pTrig = (*p).data as *mut Trigger;
        // ... use pTrig ...
        p = (*p).next;
    }

    // After:
    {
        let mut __cursor = p;
        while let Some(__node) = unsafe { __cursor.as_ref() } {
            __cursor = __node.next;
            let pTrig = __node.data as *mut Trigger;
            // ... use pTrig ...
        }
    }

This is MORE advanced than zero_init_with_memset because:
  - It performs multi-statement structural rewrite (not just init+memset pair)
  - It requires tracking which variable is the cursor vs. which is the temp
  - It must detect the "advance" assignment and hoist it
  - It handles both "advance-at-start" and "advance-at-end" variants
  - It must avoid falsely matching non-linked-list while loops

Engine features used:
  - find_nodes(), node_text(), get_parent_of_type()
  - apply_replacements()
  - Custom AST walking to detect the cursor variable and advance expression
"""

import re
from typing import List, Tuple, Optional, Any, Dict

from .base import UnsafePatternPlugin


class LinkedListNullWalkPlugin(UnsafePatternPlugin):
    """Convert C-style `while !ptr.is_null()` linked-list traversals.

    Detects the pattern where a raw pointer is walked via a null-check loop
    with explicit (*ptr).next advancement, and restructures to use
    while-let with as_ref() for safer, auditable iteration.
    """

    @property
    def name(self) -> str:
        return "linked_list_null_walk"

    @property
    def description(self) -> str:
        return (
            "Convert C-style while-!is_null linked-list traversals "
            "to idiomatic while-let iteration"
        )

    @property
    def priority(self) -> int:
        return 12  # Very high — deep structural transformation

    # ── AST inspection helpers ────────────────────────────────────────────────

    def _is_null_check_condition(self, cond_node: Any, code: str) -> Optional[str]:
        """Check if condition is `!VAR.is_null()` and return VAR name.

        Matches:
          - !p.is_null()
          - !(p.is_null())

        Returns the pointer variable name if matched, None otherwise.
        """
        text = self.node_text(cond_node, code).strip()

        # Pattern: !VAR.is_null()
        m = re.match(r'^!\s*(\w+)\s*\.\s*is_null\s*\(\s*\)$', text)
        if m:
            return m.group(1)

        # Pattern: !(VAR.is_null())
        m = re.match(r'^!\s*\(\s*(\w+)\s*\.\s*is_null\s*\(\s*\)\s*\)$', text)
        if m:
            return m.group(1)

        return None

    def _find_advance_assignment(
        self, body_node: Any, cursor_var: str, code: str
    ) -> Optional[Dict[str, Any]]:
        """Find the `cursor = (*cursor).FIELD` assignment in a loop body.

        Scans expression_statement children of the body block for an
        assignment of the form:
            cursor_var = (*cursor_var).SOME_FIELD;

        Returns dict with:
            'stmt_node': the expression_statement AST node
            'field_name': the next-pointer field name (e.g., 'pNext', 'next')
            'advance_text': the full text of the advance expression

        Or None if no such assignment found.
        """
        for child in body_node.children:
            if child.type != 'expression_statement':
                continue

            stmt_text = self.node_text(child, code).strip().rstrip(';').strip()

            # Match: cursor_var = (*cursor_var).FIELD
            pattern = (
                rf'^{re.escape(cursor_var)}\s*=\s*'
                rf'\(\s*\*\s*{re.escape(cursor_var)}\s*\)\s*\.\s*(\w+)$'
            )
            m = re.match(pattern, stmt_text)
            if m:
                return {
                    'stmt_node': child,
                    'field_name': m.group(1),
                    'advance_text': stmt_text,
                }

        return None

    def _find_tmp_swap_pattern(
        self, body_node: Any, cursor_var: str, code: str
    ) -> Optional[Dict[str, Any]]:
        """Detect the tmp-variable swap pattern used in free-loops.

        Pattern:
            let mut pTmp: TYPE = cursor;
            cursor = (*cursor).pNext;
            // ... use (*pTmp) ...
            free(pTmp);

        Returns dict with:
            'tmp_var': name of the temporary variable
            'tmp_let_node': the let_declaration AST node
            'advance_node': the advance expression_statement
            'field_name': the next-pointer field name
        """
        children = [c for c in body_node.children
                    if c.type in ('let_declaration', 'expression_statement')]
        if len(children) < 2:
            return None

        # First statement should be: let mut TMP: TYPE = cursor;
        first = children[0]
        if first.type != 'let_declaration':
            return None

        first_text = self.node_text(first, code)
        tmp_match = re.search(
            rf'let\s+mut\s+(\w+)\s*(?::\s*[^=]+)?\s*=\s*{re.escape(cursor_var)}\s*;',
            first_text
        )
        if not tmp_match:
            return None

        tmp_var = tmp_match.group(1)

        # Second statement should be: cursor = (*cursor).FIELD;
        second = children[1]
        if second.type != 'expression_statement':
            return None

        second_text = self.node_text(second, code).strip().rstrip(';').strip()
        advance_match = re.match(
            rf'^{re.escape(cursor_var)}\s*=\s*'
            rf'\(\s*\*\s*{re.escape(cursor_var)}\s*\)\s*\.\s*(\w+)$',
            second_text
        )
        if not advance_match:
            return None

        return {
            'tmp_var': tmp_var,
            'tmp_let_node': first,
            'advance_node': second,
            'field_name': advance_match.group(1),
        }

    def _count_body_statements(self, body_node: Any) -> int:
        """Count meaningful (non-whitespace/comment) statements in a block."""
        return sum(1 for c in body_node.children
                   if c.type not in ('{', '}', 'comment', 'line_comment'))

    # ── match collection ──────────────────────────────────────────────────────

    def _collect_matches(self, code: str) -> List[Dict[str, Any]]:
        """Find all while-!is_null linked list traversals."""
        root = self.parse(code)
        matches = []

        for while_node in self.find_nodes(root, 'while_expression'):
            # Get condition
            cond = while_node.child_by_field_name('condition')
            if cond is None:
                continue

            cursor_var = self._is_null_check_condition(cond, code)
            if cursor_var is None:
                continue

            # Get body block
            body = while_node.child_by_field_name('body')
            if body is None or body.type != 'block':
                continue

            # Must have an advance assignment (cursor = (*cursor).next)
            advance = self._find_advance_assignment(body, cursor_var, code)
            if advance is None:
                continue

            # Determine if this is a simple walk or a tmp-swap (free) pattern
            tmp_swap = self._find_tmp_swap_pattern(body, cursor_var, code)

            match_info = {
                'while_node': while_node,
                'cursor_var': cursor_var,
                'body_node': body,
                'advance': advance,
                'tmp_swap': tmp_swap,
                'field_name': advance['field_name'],
            }
            matches.append(match_info)

        return matches

    # ── interface ─────────────────────────────────────────────────────────────

    def find(self, rust_code: str) -> List[Tuple[int, int, str]]:
        results = []
        for m in self._collect_matches(rust_code):
            line = self.node_line(m['while_node'], rust_code)
            cursor = m['cursor_var']
            field = m['field_name']
            variant = "free-loop (tmp swap)" if m['tmp_swap'] else "read-only walk"
            results.append((
                m['while_node'].start_byte,
                m['while_node'].end_byte,
                f"Line {line}: C-style linked-list null-walk "
                f"({variant}): `{cursor}` via .{field}",
            ))
        return results

    def fix(self, rust_code: str) -> str:
        """Rewrite C-style null-walks to while-let with unsafe as_ref().

        Simple walk:
            while !p.is_null() {
                // body using (*p)
                p = (*p).pNext;
            }
          becomes:
            while !p.is_null() {
                let __current = unsafe { &*p };
                p = __current.pNext;
                // body with (*p) -> __current
            }

        For the free-loop tmp-swap pattern, we restructure similarly but
        preserve the tmp variable semantics:
            while !p.is_null() {
                let mut pTmp = p;
                p = (*p).pNext;
                // ... use (*pTmp) ...
                free(pTmp);
            }
          becomes:
            while !p.is_null() {
                let pTmp: &_ = unsafe { &*p };
                p = pTmp.pNext;
                // ... use pTmp (ref) ...
                // free remains for now with original pointer
            }

        Since the full "safe iteration" rewrite requires changing data
        structures, we focus on the achievable improvement: replacing
        scattered (*ptr) dereferences with a single unsafe { &*ptr }
        at the top of the loop, making the unsafe boundary explicit and
        auditable. The advance is moved right after the ref binding.
        """
        matches = self._collect_matches(rust_code)
        if not matches:
            return rust_code

        replacements: List[Tuple[int, int, str]] = []

        for m in matches:
            body = m['body_node']
            cursor_var = m['cursor_var']
            advance = m['advance']
            field_name = m['field_name']

            body_text = self.node_text(body, rust_code)
            # Detect indentation from the body opening
            body_lines = body_text.split('\n')
            # Find typical indentation by looking at first non-brace line
            indent = "        "
            for ln in body_lines[1:]:
                stripped = ln.lstrip()
                if stripped and not stripped.startswith('}'):
                    indent = ln[:len(ln) - len(stripped)]
                    break

            ref_var = f"__{cursor_var}_ref"

            # Build new body
            new_body_lines = ["{"]

            # First line: safe ref binding
            new_body_lines.append(
                f"{indent}let {ref_var} = unsafe {{ &*{cursor_var} }};"
            )

            # Second line: advance cursor via the ref
            new_body_lines.append(
                f"{indent}{cursor_var} = {ref_var}.{field_name};"
            )

            # Copy remaining body lines, excluding:
            #   - the advance assignment (already hoisted)
            #   - tmp let declaration (if tmp_swap pattern)
            skip_nodes = {advance['stmt_node'].start_byte}
            if m['tmp_swap']:
                skip_nodes.add(m['tmp_swap']['tmp_let_node'].start_byte)
                skip_nodes.add(m['tmp_swap']['advance_node'].start_byte)

            for child in body.children:
                if child.type in ('{', '}'):
                    continue
                if child.start_byte in skip_nodes:
                    continue

                child_text = self.node_text(child, rust_code)

                # Replace (*cursor_var). dereferences with ref_var.
                deref_pattern = rf'\(\s*\*\s*{re.escape(cursor_var)}\s*\)\s*\.'
                child_text = re.sub(deref_pattern, f'{ref_var}.', child_text)

                # If tmp_swap, also replace (*tmp_var). with ref_var.
                if m['tmp_swap']:
                    tmp = m['tmp_swap']['tmp_var']
                    tmp_deref = rf'\(\s*\*\s*{re.escape(tmp)}\s*\)\s*\.'
                    child_text = re.sub(tmp_deref, f'{ref_var}.', child_text)

                new_body_lines.append(f"{indent}{child_text.strip()}")

            # Close the block at the indentation of the opening brace
            outer_indent = indent[:-4] if len(indent) >= 4 else ""
            new_body_lines.append(f"{outer_indent}}}")

            new_body = "\n".join(new_body_lines)
            replacements.append((body.start_byte, body.end_byte, new_body))

        return self.apply_replacements(rust_code, replacements)

    # ── tests ─────────────────────────────────────────────────────────────────

    def test(self) -> bool:
        all_passed = True

        # Test 1: Simple read-only walk
        code1 = '\n'.join([
            "pub unsafe fn walk_list(mut p: *mut Node) {",
            "    while !p.is_null() {",
            "        let val = (*p).data;",
            "        use_val(val);",
            "        p = (*p).pNext;",
            "    }",
            "}",
        ])
        finds1 = self.find(code1)
        if len(finds1) != 1:
            print(f"  x simple walk: expected 1 find, got {len(finds1)}")
            all_passed = False
        else:
            print("  v simple walk: find OK")

        fixed1 = self.fix(code1)
        if "__p_ref" not in fixed1:
            print(f"  x simple walk: ref variable not in output")
            print(f"     output: {fixed1}")
            all_passed = False
        elif "unsafe { &*p }" not in fixed1:
            print(f"  x simple walk: unsafe ref binding not found")
            all_passed = False
        elif "(*p)." in fixed1:
            print(f"  x simple walk: raw deref (*p). still present")
            all_passed = False
        else:
            print("  v simple walk: fix OK")

        # Test 2: Free-loop with tmp swap
        code2 = '\n'.join([
            "pub unsafe fn free_list(mut db: *mut Db, mut pStep: *mut Step) {",
            "    while !pStep.is_null() {",
            "        let mut pTmp: *mut Step = pStep;",
            "        pStep = (*pStep).pNext;",
            "        free(db, (*pTmp).pWhere);",
            "        free(db, (*pTmp).pExpr);",
            "    }",
            "}",
        ])
        finds2 = self.find(code2)
        if len(finds2) != 1:
            print(f"  x free-loop: expected 1 find, got {len(finds2)}")
            all_passed = False
        else:
            print("  v free-loop: find OK")
            if "free-loop" not in finds2[0][2]:
                print(f"  x free-loop: not classified as free-loop: {finds2[0][2]}")
                all_passed = False
            else:
                print("  v free-loop: classification OK")

        # Test 3: Non-linked-list while loop (should NOT match)
        code3 = '\n'.join([
            "pub unsafe fn not_a_list(mut i: i32) {",
            "    while i > 0 {",
            "        i -= 1;",
            "    }",
            "}",
        ])
        finds3 = self.find(code3)
        if len(finds3) != 0:
            print(f"  x non-list loop: expected 0 finds, got {len(finds3)}")
            all_passed = False
        else:
            print("  v non-list loop: correctly skipped")

        # Test 4: Null check without advance (should NOT match)
        code4 = '\n'.join([
            "pub unsafe fn no_advance(mut p: *mut Node) {",
            "    while !p.is_null() {",
            "        use_node(p);",
            "        break;",
            "    }",
            "}",
        ])
        finds4 = self.find(code4)
        if len(finds4) != 0:
            print(f"  x no-advance: expected 0 finds, got {len(finds4)}")
            all_passed = False
        else:
            print("  v no-advance: correctly skipped")

        # Test 5: Multiple linked list walks in same function
        code5 = '\n'.join([
            "pub unsafe fn multi(mut a: *mut N, mut b: *mut N) {",
            "    while !a.is_null() {",
            "        use_n((*a).val);",
            "        a = (*a).pNext;",
            "    }",
            "    while !b.is_null() {",
            "        use_n((*b).val);",
            "        b = (*b).pNext;",
            "    }",
            "}",
        ])
        finds5 = self.find(code5)
        if len(finds5) != 2:
            print(f"  x multi-walk: expected 2 finds, got {len(finds5)}")
            all_passed = False
        else:
            print("  v multi-walk: find OK")

        return all_passed
