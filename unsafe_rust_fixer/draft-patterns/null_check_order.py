"""Plugin: Detects null checks AFTER pointer dereference (wrong order)."""
from typing import List, Tuple
import re
from .base import UnsafePatternPlugin


class NullCheckOrderPlugin(UnsafePatternPlugin):
    """Detect dereferencing pointers before null checks.

    Pattern: dereferencing a pointer, then checking if it's null.
    This is a logic error - should check null BEFORE dereferencing.
    """

    @property
    def name(self) -> str:
        return "null_check_order"

    @property
    def description(self) -> str:
        return "Dereferencing before null check (wrong order)"

    @property
    def priority(self) -> int:
        return 8  # High priority - safety issue

    def find(self, rust_code: str) -> List[Tuple[int, int, str]]:
        """Find dereference operations followed by null checks on same pointer."""
        results = []

        # Look for patterns like: (*ptr).field ... if ptr.is_null()
        # This is regex-based as it requires tracking variable scope
        lines = rust_code.split('\n')

        for i, line in enumerate(lines):
            # Pattern: dereference like (*var) or *var
            deref_match = re.search(r'\(\*(\w+)\)', line)
            if not deref_match:
                deref_match = re.search(r'(?:^|\s)\*(\w+)\b', line)

            if deref_match:
                var_name = deref_match.group(1)
                # Look forward in next few lines for null check on same var
                for j in range(i + 1, min(i + 10, len(lines))):
                    if re.search(rf'{var_name}\.is_null\(\)|{var_name} == null|{var_name} == NULL', lines[j]):
                        line_num = i + 1
                        results.append((
                            sum(len(l) + 1 for l in lines[:i]),
                            sum(len(l) + 1 for l in lines[:i+1]),
                            f"Line {line_num}: Dereferencing {var_name} before null check (line {j+1})"
                        ))
                        break

        return results

    def fix(self, rust_code: str) -> str:
        """Dereference operations cannot be auto-fixed - requires manual reordering."""
        # This is too dangerous to auto-fix
        return rust_code

    def test(self) -> bool:
        """Test null check order detection."""
        test_cases = [
            {
                "name": "dereference then null check",
                "code": "let x = (*ptr).field;\nif ptr.is_null() { }",
                "should_find": True,
                "expected_matches": 1,
            },
            {
                "name": "null check then dereference",
                "code": "if ptr.is_null() { }\nlet x = (*ptr).field;",
                "should_find": False,
                "expected_matches": 0,
            },
            {
                "name": "no dereference",
                "code": "if ptr.is_null() { }",
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
