"""Plugin: Remove no-op .offset(0) calls on raw pointers.

Transpiled C code often emits patterns like:
    (&raw mut arr as *mut T).offset(0 as ::core::ffi::c_int as isize)

Calling .offset(0) is a mathematical identity: the pointer is unchanged.
This plugin detects and removes those calls, replacing the whole expression
with just the receiver.

Before:
    (&raw mut (*pTabList).a as *mut SrcItem).offset(0 as ::core::ffi::c_int as isize)

After:
    (&raw mut (*pTabList).a as *mut SrcItem)

Note: In tree-sitter-rust, method calls like p.offset(0) are represented as
  call_expression { function: field_expression { value: p, field: offset }, arguments }
NOT as method_call_expression.
"""
from typing import List, Tuple, Optional, Any
from .base import UnsafePatternPlugin


class ZeroOffsetElisionPlugin(UnsafePatternPlugin):
    """Remove .offset(0) method calls — they are no-ops on raw pointers.

    A call ptr.offset(0) always returns ptr unchanged. The C-to-Rust
    transpiler emits these when accessing the first element of a C array
    via pointer arithmetic. They add noise without any semantic value.
    """

    @property
    def name(self) -> str:
        return "zero_offset_elision"

    @property
    def description(self) -> str:
        return "Remove no-op .offset(0) calls on raw pointers"

    @property
    def priority(self) -> int:
        return 8  # High — safe, mechanical, high signal/noise improvement

    # ── helpers ──────────────────────────────────────────────────────────────

    def _get_offset_receiver(
        self, call_node: Any, code: str
    ) -> Optional[Any]:
        """If call_node is a .offset(ZERO) call, return the receiver node.

        In this grammar, p.offset(0) is:
          call_expression
            function: field_expression
              value:  <receiver>
              field:  field_identifier("offset")
            arguments: arguments
              integer_literal("0")

        Returns the receiver node if this is .offset(zero), else None.
        """
        fn_node = call_node.child_by_field_name('function')
        if fn_node is None or fn_node.type != 'field_expression':
            return None

        # field_expression must have field "offset"
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

        # arguments must contain exactly one zero-value arg
        args_node = call_node.child_by_field_name('arguments')
        if args_node is None:
            return None

        arg_nodes = [c for c in args_node.children
                     if c.type not in ('(', ')', ',', 'comment')]
        if len(arg_nodes) != 1:
            return None

        if not self.is_zero_value_node(arg_nodes[0], code):
            return None

        return receiver

    # ── interface ─────────────────────────────────────────────────────────────

    def find(self, rust_code: str) -> List[Tuple[int, int, str]]:
        """Find call_expression nodes that are .offset(0)."""
        results = []
        root = self.parse(rust_code)

        for call in self.find_nodes(root, 'call_expression'):
            receiver = self._get_offset_receiver(call, rust_code)
            if receiver is not None:
                line = self.node_line(call, rust_code)
                call_text = self.node_text(call, rust_code)
                results.append((
                    call.start_byte,
                    call.end_byte,
                    f"Line {line}: .offset(0) is a no-op — receiver unchanged: "
                    f"{call_text[:70]}",
                ))

        return results

    def fix(self, rust_code: str) -> str:
        """Replace each .offset(0) call with just its receiver expression."""
        root = self.parse(rust_code)
        replacements = []

        for call in self.find_nodes(root, 'call_expression'):
            receiver = self._get_offset_receiver(call, rust_code)
            if receiver is not None:
                receiver_text = self.node_text(receiver, rust_code)
                replacements.append((call.start_byte, call.end_byte, receiver_text))

        return self.apply_replacements(rust_code, replacements)

    def test(self) -> bool:
        """Self-contained tests for zero_offset_elision."""
        test_cases = [
            {
                "name": "simple offset(0)",
                "code": "fn f(p: *mut u8) { let q = p.offset(0); }",
                "expected_finds": 1,
                "fixed_contains": "let q = p;",
                "fixed_not_contains": ".offset(",
            },
            {
                "name": "offset with cast noise",
                "code": (
                    "fn f(p: *mut u8) { "
                    "let q = p.offset(0 as ::core::ffi::c_int as isize); }"
                ),
                "expected_finds": 1,
                "fixed_contains": "let q = p;",
                "fixed_not_contains": ".offset(",
            },
            {
                "name": "non-zero offset — leave alone",
                "code": "fn f(p: *mut u8) { let q = p.offset(1); }",
                "expected_finds": 0,
                "fixed_contains": ".offset(1)",
                "fixed_not_contains": None,
            },
            {
                "name": "offset with variable — leave alone",
                "code": "fn f(p: *mut u8, i: isize) { let q = p.offset(i); }",
                "expected_finds": 0,
                "fixed_contains": ".offset(i)",
                "fixed_not_contains": None,
            },
            {
                "name": "cast receiver with offset(0)",
                "code": (
                    "fn f(arr: [u8; 4]) -> *mut u8 { "
                    "unsafe { (&raw mut arr as *mut u8).offset(0 as i32 as isize) } }"
                ),
                "expected_finds": 1,
                "fixed_contains": "(&raw mut arr as *mut u8)",
                "fixed_not_contains": ".offset(",
            },
        ]

        all_passed = True
        for tc in test_cases:
            finds = self.find(tc["code"])
            if len(finds) != tc["expected_finds"]:
                print(f"  ✗ {tc['name']}: expected {tc['expected_finds']} finds, "
                      f"got {len(finds)}")
                all_passed = False
            else:
                print(f"  ✓ {tc['name']}: find OK")

            fixed = self.fix(tc["code"])
            if tc["fixed_contains"] and tc["fixed_contains"] not in fixed:
                print(f"  ✗ {tc['name']}: fixed output missing "
                      f"'{tc['fixed_contains']}'")
                print(f"       got: {fixed!r}")
                all_passed = False
            elif tc["fixed_not_contains"] and tc["fixed_not_contains"] in fixed:
                print(f"  ✗ {tc['name']}: fixed output still contains "
                      f"'{tc['fixed_not_contains']}'")
                all_passed = False
            else:
                print(f"  ✓ {tc['name']}: fix OK")

        return all_passed
