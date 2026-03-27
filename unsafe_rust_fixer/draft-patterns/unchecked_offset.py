"""Plugin: Detects pointer offset without prior bounds checking."""
from typing import List, Tuple
from .base import UnsafePatternPlugin


class UncheckedOffsetPlugin(UnsafePatternPlugin):
    """Detect pointer .offset() calls without prior bounds validation.

    Finds .offset() usage that may lack bounds checking.
    """

    @property
    def name(self) -> str:
        return "unchecked_offset"

    @property
    def description(self) -> str:
        return "Pointer .offset() calls without bounds checking"

    @property
    def priority(self) -> int:
        return 3

    def find(self, rust_code: str) -> List[Tuple[int, int, str]]:
        """Find .offset() calls."""
        results = []
        root = self.parse(rust_code)

        # Find all field_expression nodes (for .offset calls)
        call_exprs = self.find_nodes(root, 'call_expression')

        for call_expr in call_exprs:
            text = self.node_text(call_expr, rust_code)

            # Look for .offset( pattern
            if '.offset(' in text:
                line = self.node_line(call_expr, rust_code)

                # Check if in unsafe block
                if not self.has_unsafe_block(call_expr):
                    results.append((
                        call_expr.start_byte,
                        call_expr.end_byte,
                        f"Line {line}: Unchecked pointer offset: {text[:60]}"
                    ))

        return results

    def fix(self, rust_code: str) -> str:
        """Wrap unchecked offsets in unsafe blocks.

        Note: This is a minimal fix. Proper fix requires bounds checking.
        """
        matches = self.find(rust_code)

        if not matches:
            return rust_code

        for start, end, _ in sorted(matches, reverse=True):
            offset_call = rust_code[start:end]
            wrapped = f"unsafe {{ {offset_call} }}"
            rust_code = rust_code[:start] + wrapped + rust_code[end:]

        return rust_code

    def test(self) -> bool:
        """Test unchecked offset detection."""
        test_cases = [
            {
                "name": "unprotected offset",
                "code": "let x = ptr.offset(i as isize);",
                "should_find": True,
                "expected_matches": 1,
            },
            {
                "name": "protected offset",
                "code": "let x = unsafe { ptr.offset(i as isize) };",
                "should_find": False,
                "expected_matches": 0,
            },
            {
                "name": "no offset calls",
                "code": "let x = ptr;",
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
