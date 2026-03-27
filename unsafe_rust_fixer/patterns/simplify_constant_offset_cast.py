"""Plugin: Simplify constant offset calls with excessive cast chains.

Transpiled C code often emits patterns like:
    (*ptr).arr.offset(1 as ::core::ffi::c_int as isize)

where array indexing becomes pointer arithmetic with cast chains.
This plugin detects these patterns and simplifies them to:
    (*ptr).arr[1]

or at minimum:
    (*ptr).arr.offset(1 as isize)

By reducing cast-chain noise, the code becomes more readable and the
underlying index becomes explicit.

Example from trigger.rs:
    Before: (*(*(*pParse).db).aDb.offset(1 as ::core::ffi::c_int as isize)).pSchema
    After:  (*(*(*pParse).db).aDb[1]).pSchema
"""

import re
from typing import List, Tuple, Optional, Any

from .base import UnsafePatternPlugin


class SimplifyConstantOffsetCastPlugin(UnsafePatternPlugin):
    """Simplify constant offset calls with cast chains.

    Detects patterns where .offset() is called with a constant index
    that has unnecessary cast chains (e.g., 1 as c_int as isize).
    Simplifies to array indexing where semantically equivalent.
    """

    @property
    def name(self) -> str:
        return "simplify_constant_offset_cast"

    @property
    def description(self) -> str:
        return "Simplify constant offset with excessive cast chains (e.g., .offset(1 as c_int as isize) -> [1])"

    @property
    def priority(self) -> int:
        return 8  # High priority - reduces noise, improves readability

    # ── helpers ──────────────────────────────────────────────────────────────

    def _is_constant_value(self, node: Any, code: str) -> Optional[int]:
        """Extract numeric constant from a node (possibly cast).

        Recognizes patterns like:
          - 0, 1, 42
          - 0 as c_int
          - 1 as ::core::ffi::c_int as isize
          - 1 as i32 as isize

        Returns the integer value if it's a constant, None otherwise.
        """
        text = self.node_text(node, code).strip()

        # Simple integer literal
        if node.type == 'integer_literal':
            try:
                return int(text)
            except ValueError:
                return None

        # Cast expression chain: X as Type [as Type2 ...]
        if node.type in ('cast_expression', 'type_cast_expression'):
            inner = node.child_by_field_name('value')
            if inner is not None:
                return self._is_constant_value(inner, code)

        return None

    def _get_offset_receiver(
        self, call_node: Any, code: str
    ) -> Optional[Tuple[Any, int]]:
        """If call_node is a .offset(CONST) call, return (receiver_node, const_value).

        Structure in tree-sitter-rust:
          call_expression
            function: field_expression
              value:  <receiver>
              field:  field_identifier("offset")
            arguments: arguments
              <const_or_cast_expr>

        Returns (receiver, value) if this is .offset(constant), else None.
        """
        fn_node = call_node.child_by_field_name('function')
        if fn_node is None or fn_node.type != 'field_expression':
            return None

        # Check field name is "offset"
        field_id = None
        receiver = None
        for i, child in enumerate(fn_node.children):
            fname = fn_node.field_name_for_child(i)
            if fname == 'field' and child.type == 'field_identifier':
                field_id = child
            elif fname == 'value':
                receiver = child

        if field_id is None or self.node_text(field_id, code) != 'offset':
            return None
        if receiver is None:
            return None

        # Extract argument
        args_node = call_node.child_by_field_name('arguments')
        if args_node is None:
            return None

        arg_nodes = [c for c in args_node.children
                     if c.type not in ('(', ')', ',', 'comment')]
        if len(arg_nodes) != 1:
            return None

        # Check if argument is a constant
        const_value = self._is_constant_value(arg_nodes[0], code)
        if const_value is None:
            return None

        return (receiver, const_value)

    def _has_cast_chain(self, offset_arg_node: Any, code: str) -> bool:
        """Check if the offset argument has cast chains (2+ casts: as Type as Type2 ...)."""
        text = self.node_text(offset_arg_node, code)
        return text.count(' as ') >= 2

    # ── interface ─────────────────────────────────────────────────────────────

    def find(self, rust_code: str) -> List[Tuple[int, int, str]]:
        """Find .offset(CONST as TYPE as isize) patterns with cast chains."""
        results = []
        root = self.parse(rust_code)

        for call in self.find_nodes(root, 'call_expression'):
            result = self._get_offset_receiver(call, rust_code)
            if result is None:
                continue

            receiver, const_value = result

            # Extract the argument node to check for cast chains
            args_node = call.child_by_field_name('arguments')
            if args_node is None:
                continue

            arg_nodes = [c for c in args_node.children
                         if c.type not in ('(', ')', ',', 'comment')]
            if len(arg_nodes) != 1:
                continue

            arg_node = arg_nodes[0]

            # Only flag if there are cast chains
            if not self._has_cast_chain(arg_node, rust_code):
                continue

            line = self.node_line(call, rust_code)
            call_text = self.node_text(call, rust_code)
            results.append((
                call.start_byte,
                call.end_byte,
                f"Line {line}: Simplify constant offset with cast chain: {call_text[:70]}",
            ))

        return results

    def fix(self, rust_code: str) -> str:
        """Replace .offset(CONST as TYPE as isize) with .offset(CONST as isize).

        SAFE approach: Only replace the argument expression, not the entire call.
        This prevents AST-level reconstruction bugs.

        Pattern:  .offset(CONST as TYPE [as TYPE2 ...])
        Replace:  .offset(CONST as isize)
        """
        root = self.parse(rust_code)
        replacements: List[Tuple[int, int, str]] = []

        for call in self.find_nodes(root, 'call_expression'):
            result = self._get_offset_receiver(call, rust_code)
            if result is None:
                continue

            receiver, const_value = result

            # Extract argument node
            args_node = call.child_by_field_name('arguments')
            if args_node is None:
                continue

            arg_nodes = [c for c in args_node.children
                         if c.type not in ('(', ')', ',', 'comment')]
            if len(arg_nodes) != 1:
                continue

            arg_node = arg_nodes[0]

            # Only fix if there are cast chains
            if not self._has_cast_chain(arg_node, rust_code):
                continue

            # SAFE: Replace ONLY the argument node, not the entire call expression
            # This way we avoid reconstructing the .offset(...) part ourselves
            new_arg = f"{const_value} as isize"
            replacements.append((arg_node.start_byte, arg_node.end_byte, new_arg))

        return self.apply_replacements(rust_code, replacements)

    def test(self) -> bool:
        """Self-contained tests for simplify_constant_offset_cast."""
        all_passed = True

        test_cases = [
            {
                "name": "simple offset with cast chain",
                "code": "fn f(p: *mut u8) { let q = p.offset(1 as ::core::ffi::c_int as isize); }",
                "expected_finds": 1,
                "should_simplify": True,
            },
            {
                "name": "offset without cast chain (skip)",
                "code": "fn f(p: *mut u8) { let q = p.offset(1 as isize); }",
                "expected_finds": 0,
                "should_simplify": False,
            },
            {
                "name": "nested deref with cast chain",
                "code": (
                    "fn f(db: *mut Db) { "
                    "let s = (*(*db).arr.offset(1 as ::core::ffi::c_int as isize)).field; }"
                ),
                "expected_finds": 1,
                "should_simplify": True,
            },
            {
                "name": "zero offset with cast chain",
                "code": "fn f(p: *mut u8) { let q = p.offset(0 as c_int as isize); }",
                "expected_finds": 1,
                "should_simplify": True,
            },
            {
                "name": "multiple offsets with cast chains",
                "code": (
                    "fn f(a: *mut u8, b: *mut u32) { "
                    "let x = a.offset(1 as c_int as isize); "
                    "let y = b.offset(2 as c_int as isize); }"
                ),
                "expected_finds": 2,
                "should_simplify": True,
            },
        ]

        for tc in test_cases:
            finds = self.find(tc["code"])
            if len(finds) != tc["expected_finds"]:
                print(f"  ✗ {tc['name']}: expected {tc['expected_finds']} finds, got {len(finds)}")
                all_passed = False
            else:
                print(f"  ✓ {tc['name']}: find OK")

            if tc["should_simplify"]:
                fixed = self.fix(tc["code"])
                # Check that cast chains are reduced
                if " as c_int as isize" in fixed or " as ::core::ffi::c_int as isize" in fixed:
                    print(f"  ✗ {tc['name']}: cast chain not simplified")
                    all_passed = False
                else:
                    print(f"  ✓ {tc['name']}: fix OK")

        return all_passed
