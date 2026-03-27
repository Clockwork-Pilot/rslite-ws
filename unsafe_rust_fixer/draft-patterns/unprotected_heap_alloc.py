"""Plugin: Detects unprotected heap allocation calls."""
from typing import List, Tuple
from .base import UnsafePatternPlugin


class UnprotectedHeapAllocPlugin(UnsafePatternPlugin):
    """Detect heap allocation calls without unsafe blocks.

    Finds malloc, realloc, alloc, and similar heap operations
    that should be wrapped in unsafe blocks.
    """

    @property
    def name(self) -> str:
        return "unprotected_heap_alloc"

    @property
    def description(self) -> str:
        return "Heap allocation calls (malloc, realloc) without unsafe blocks"

    @property
    def priority(self) -> int:
        return 6

    # Common allocation function patterns
    ALLOC_PATTERNS = [
        'malloc',
        'realloc',
        'calloc',
        'alloc',
        'Box::new',
        'Vec::with_capacity',
    ]

    def find(self, rust_code: str) -> List[Tuple[int, int, str]]:
        """Find heap allocation calls."""
        results = []
        root = self.parse(rust_code)

        call_exprs = self.find_nodes(root, 'call_expression')

        for call_expr in call_exprs:
            text = self.node_text(call_expr, rust_code)

            # Check for allocation patterns
            for pattern in self.ALLOC_PATTERNS:
                if pattern in text:
                    line = self.node_line(call_expr, rust_code)

                    if not self.has_unsafe_block(call_expr):
                        results.append((
                            call_expr.start_byte,
                            call_expr.end_byte,
                            f"Line {line}: Unprotected heap allocation: {text[:60]}"
                        ))
                    break

        return results

    def fix(self, rust_code: str) -> str:
        """Wrap unprotected allocations in unsafe blocks.

        Note: Some allocations (Box::new, Vec) are safe in Rust.
        Manual review recommended.
        """
        matches = self.find(rust_code)

        if not matches:
            return rust_code

        for start, end, _ in sorted(matches, reverse=True):
            call_text = rust_code[start:end]
            wrapped = f"unsafe {{ {call_text} }}"
            rust_code = rust_code[:start] + wrapped + rust_code[end:]

        return rust_code

    def test(self) -> bool:
        """Test unprotected heap allocation detection."""
        test_cases = [
            {
                "name": "malloc call",
                "code": "let ptr = malloc(size);",
                "should_find": True,
                "expected_matches": 1,
            },
            {
                "name": "realloc call",
                "code": "let ptr = realloc(ptr, new_size);",
                "should_find": True,
                "expected_matches": 1,
            },
            {
                "name": "protected malloc",
                "code": "let ptr = unsafe { malloc(size) };",
                "should_find": False,
                "expected_matches": 0,
            },
            {
                "name": "safe allocation",
                "code": "let v = vec![1, 2, 3];",
                "should_find": False,
                "expected_matches": 0,
            },
            {
                "name": "no allocations",
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
