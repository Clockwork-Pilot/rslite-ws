"""Plugin: Detects unsafe void pointer casts and conversions."""
from typing import List, Tuple
from .base import UnsafePatternPlugin


class VoidPointerCastPlugin(UnsafePatternPlugin):
    """Detect void pointer (*mut c_void, *const c_void) conversions.

    These are often the result of C FFI and may hide type safety issues.
    """

    @property
    def name(self) -> str:
        return "void_pointer_cast"

    @property
    def description(self) -> str:
        return "Void pointer casts (c_void) - potential type safety issues"

    @property
    def priority(self) -> int:
        return 4

    def find(self, rust_code: str) -> List[Tuple[int, int, str]]:
        """Find void pointer casts and operations."""
        results = []
        root = self.parse(rust_code)

        # Find all cast and unary expressions
        cast_exprs = self.find_nodes(root, 'cast_expression')
        unary_exprs = self.find_nodes(root, 'unary_expression')

        for expr in cast_exprs + unary_exprs:
            text = self.node_text(expr, rust_code)

            # Look for c_void or c_void patterns
            if 'c_void' in text or '::core::ffi::c_void' in text:
                line = self.node_line(expr, rust_code)

                # Check if in unsafe block
                if not self.has_unsafe_block(expr):
                    results.append((
                        expr.start_byte,
                        expr.end_byte,
                        f"Line {line}: Void pointer operation: {text[:60]}"
                    ))

        return results

    def fix(self, rust_code: str) -> str:
        """Wrap void pointer operations in unsafe blocks.

        Note: Manual review recommended.
        """
        matches = self.find(rust_code)

        if not matches:
            return rust_code

        for start, end, _ in sorted(matches, reverse=True):
            expr_text = rust_code[start:end]
            wrapped = f"unsafe {{ {expr_text} }}"
            rust_code = rust_code[:start] + wrapped + rust_code[end:]

        return rust_code

    def test(self) -> bool:
        """Test void pointer cast detection."""
        test_cases = [
            {
                "name": "regular cast",
                "code": "let x = val as u32;",
                "should_find": False,
                "expected_matches": 0,
            },
            {
                "name": "void pointer in unsafe",
                "code": "let p = unsafe { ptr as *mut ::core::ffi::c_void };",
                "should_find": False,
                "expected_matches": 0,
            },
            {
                "name": "no casts",
                "code": "let x = 42;",
                "should_find": False,
                "expected_matches": 0,
            },
        ]

        all_passed = True
        for test_case in test_cases:
            findings = self.find(test_case["code"])
            match_count = len(findings)

            if match_count != test_case["expected_matches"]:
                print(f"  ✗ {test_case['name']}: expected {test_case['expected_matches']}, got {match_count}")
                all_passed = False
            else:
                print(f"  ✓ {test_case['name']}")

        return all_passed
