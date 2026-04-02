"""Plugin: Fix clippy warnings for needless_return, assign_op_pattern, and collapsible_else_if.

Detects and fixes three common clippy patterns:
  1. needless_return: `return <expr>;` as last statement → `<expr>`
  2. assign_op_pattern: `VAR = VAR OP EXPR;` → `VAR OP= EXPR;`
  3. collapsible_else_if: `else { if COND { ... } }` → `else if COND { ... }`

Using tree-sitter for accurate AST-based detection and transformation.

Most common clippy warnings:
  - 2756 needless_return instances
  - 385 assign_op_pattern instances
  - 226 collapsible_else_if instances
"""

from typing import List, Tuple, Optional, Any
import re

try:
    from .base import UnsafePatternPlugin
except ImportError:
    # Fallback for when loaded as a script
    from base import UnsafePatternPlugin  # type: ignore


class ClippyWarningPatternsPlugin(UnsafePatternPlugin):
    """Fix clippy warnings: needless_return, assign_op_pattern, collapsible_else_if."""

    @property
    def name(self) -> str:
        return "clippy_warning_patterns"

    @property
    def description(self) -> str:
        return (
            "Fix clippy warnings: remove needless return statements, "
            "simplify compound assignments (a = a + b → a += b), "
            "and collapse else-if chains (else { if ... } → else if ...)"
        )

    @property
    def priority(self) -> int:
        return 15  # Very high priority - 2700+ instances total

    # ========== Sub-pattern 1: Needless Return ==========

    def _is_last_statement_in_block(self, stmt_node: Any, code: str) -> bool:
        """Check if statement is the last statement in the function body.

        For a return statement to be needless, it must be:
        1. The last statement in the function body
        2. Not in any conditional/loop (if/match/etc) - those must keep return
        3. Directly in the function's outermost block

        This method is very conservative to avoid corrupting code.
        """
        if not stmt_node or not stmt_node.parent:
            return False

        # Check: Is stmt_node directly in an expression_statement?
        # (The return_expression itself is a child of expression_statement)
        parent = stmt_node.parent
        if parent.type != 'expression_statement':
            return False

        # Walk up from the expression_statement to find the containing function
        # IMPORTANT: We only accept returns that are in the direct function body,
        # NOT in any nested block (if, loop, match, etc.)
        current = parent.parent
        depth = 0  # Count how many blocks we've seen
        found_function = False

        while current:
            # Count depth through blocks
            if current.type == 'block':
                depth += 1

                # Check if this block's parent is a function
                block_parent = current.parent
                if block_parent and block_parent.type in ('function_item', 'closure_expression',
                                                          'async_block', 'closure_expression'):
                    # This is a function body block
                    if depth == 1:
                        # Direct child of function - check if last statement
                        statements = [
                            c for c in current.children
                            if c.type in ('expression_statement', 'return_expression',
                                         'let_declaration', 'const_item', 'static_item',
                                         'if_expression', 'match_expression', 'loop_expression',
                                         'while_expression', 'for_expression')
                            and c.type != 'comment'
                        ]
                        # This is the last statement if it's the last in the function body
                        if statements and statements[-1] == parent:
                            return True
                    # If depth > 1, it means we're in a nested block, don't convert
                    return False

                # If this is a block but not a function body, check what it belongs to
                if block_parent:
                    if block_parent.type in ('if_expression', 'match_expression', 'match_arm',
                                           'loop_expression', 'while_expression', 'for_expression',
                                           'else_clause', 'async_block'):
                        # This is a nested block in a conditional/loop
                        # Don't remove returns from here
                        return False

            # If we hit any other type that's not a block, stop
            if current.type not in ('block', 'program', 'source_file'):
                # We hit something else before finding a function
                return False

            current = current.parent

        return False

    def _is_return_statement(self, node: Any, code: str) -> bool:
        """Check if node is a return statement with an expression."""
        if node.type != 'return_expression':
            return False

        # Must have an expression (not just bare return)
        # return_expression has child: return_keyword and optionally expression
        for child in node.children:
            if child.type != 'return' and child.type != 'comment':
                return True
        return False

    def _extract_return_expr(self, return_node: Any, code: str) -> str:
        """Extract the expression from a return statement."""
        expr = None
        for child in return_node.children:
            if child.type not in ('return', 'comment', ';'):
                expr = child
                break

        if expr:
            return self.node_text(expr, code).strip()
        return ""

    def _find_needless_returns(self, rust_code: str) -> List[Tuple[int, int, str]]:
        """Find return statements that can be converted to bare expressions."""
        results = []
        root = self.parse(rust_code)

        # Find all return expressions
        return_nodes = self.find_nodes(root, 'return_expression')

        for return_node in return_nodes:
            # Check if this return has an expression
            if not self._is_return_statement(return_node, rust_code):
                continue

            # Check if it's the last statement in a block
            stmt_parent = self.get_statement_node(return_node)
            if not stmt_parent:
                # The return itself might be in an expression context
                continue

            if self._is_last_statement_in_block(return_node, rust_code):
                expr_text = self._extract_return_expr(return_node, rust_code)
                if expr_text:
                    results.append((
                        return_node.start_byte,
                        return_node.end_byte,
                        f"Remove needless return: '{expr_text}'"
                    ))

        return results

    # ========== Sub-pattern 2: Assign Op Pattern ==========

    def _are_expressions_equal(self, left_text: str, right_text: str) -> bool:
        """Check if two expression texts are the same (ignoring whitespace)."""
        return left_text.strip() == right_text.strip()

    def _extract_op_from_binary(self, binary_node: Any, code: str) -> str:
        """Extract the operator from a binary expression.

        Returns: '+', '-', '*', '/', '%', '&', '|', '^', '<<', '>>', etc.
        """
        # Binary expression structure: left operator right
        for child in binary_node.children:
            if child.type in ('binary_operator', '+', '-', '*', '/', '%', '&', '|', '^', '<<', '>>'):
                return self.node_text(child, code).strip()
        return ""

    def _is_simple_identifier_or_path(self, node: Any, code: str) -> bool:
        """Check if node is a simple identifier or path (no complex expressions)."""
        text = self.node_text(node, code).strip()
        # Simple check: no function calls, indexing, etc.
        return not any(c in text for c in ('(', '[', '{'))

    def _find_assign_ops(self, rust_code: str) -> List[Tuple[int, int, str]]:
        """Find assignments like `a = a + 1` that can become `a += 1`."""
        results = []
        root = self.parse(rust_code)

        # Find all binary assignments
        assignments = self.find_nodes_by_predicate(
            root,
            lambda n: n.type == 'assignment_expression'
        )

        for assign_node in assignments:
            # Assignment structure: left = right
            left_node = None
            right_node = None

            for child in assign_node.children:
                if left_node is None and child.type != '=':
                    left_node = child
                elif child.type == '=' and right_node is None:
                    # Next non-= child is right
                    continue
                elif right_node is None and left_node is not None:
                    right_node = child
                    break

            if not (left_node and right_node):
                continue

            # Right side should be a binary expression
            if right_node.type != 'binary_expression':
                continue

            # Extract: binary_expression structure is left OP right
            bin_children = [c for c in right_node.children if c.type != 'comment']
            if len(bin_children) < 3:
                continue

            bin_left = bin_children[0]
            bin_op = bin_children[1]
            bin_right = bin_children[2]

            op_text = self.node_text(bin_op, rust_code).strip()
            left_text = self.node_text(left_node, rust_code).strip()
            bin_left_text = self.node_text(bin_left, rust_code).strip()
            bin_right_text = self.node_text(bin_right, rust_code).strip()

            # Check if left side matches the left side of binary expression
            if self._are_expressions_equal(left_text, bin_left_text):
                # Check if operator is one we can convert
                if op_text in ('+', '-', '*', '/', '%', '&', '|', '^', '<<', '>>'):
                    results.append((
                        assign_node.start_byte,
                        assign_node.end_byte,
                        f"Simplify assignment: use {op_text}= operator"
                    ))

        return results

    # ========== Sub-pattern 3: Collapsible Else If ==========

    def _find_collapsible_else_if(self, rust_code: str) -> List[Tuple[int, int, str]]:
        """Find else blocks containing only a single if expression (using tree-sitter for accuracy)."""
        results = []

        try:
            root = self.parse(rust_code)
        except Exception:
            return results

        if_nodes = self.find_nodes(root, 'if_expression')

        for if_node in if_nodes:
            # Look for else_clause
            else_clause = None
            for child in if_node.children:
                if child.type == 'else_clause':
                    else_clause = child
                    break

            if not else_clause:
                continue

            # Check if else_clause has a block (else { ... }) rather than else if
            block = None
            for child in else_clause.children:
                if child.type == 'block':
                    block = child
                    break

            if not block:
                # else if case - already in correct form
                continue

            # Check what's inside the block
            non_comment_children = [c for c in block.children if c.type not in ('comment', '{', '}')]

            if len(non_comment_children) != 1:
                continue

            stmt = non_comment_children[0]

            # Check if it's an if_expression directly or wrapped in expression_statement
            if_expr = None
            if stmt.type == 'if_expression':
                if_expr = stmt
            elif stmt.type == 'expression_statement':
                for child in stmt.children:
                    if child.type == 'if_expression':
                        if_expr = child
                        break

            if not if_expr:
                continue

            # This is genuinely collapsible
            results.append((
                else_clause.start_byte,
                block.end_byte,
                "Collapse else-if: can be simplified to else if"
            ))

        return results

    def _find_collapsible_else_if_DISABLED(self, rust_code: str) -> List[Tuple[int, int, str]]:
        """Original implementation - disabled due to implementation difficulty."""
        results = []
        root = self.parse(rust_code)

        # Find all if expressions
        if_nodes = self.find_nodes(root, 'if_expression')

        for if_node in if_nodes:
            # Look for else_clause in children
            else_clause = None
            for child in if_node.children:
                if child.type == 'else_clause':
                    else_clause = child
                    break

            if not else_clause:
                continue

            # else_clause structure: else block OR else if_expression
            # We want: else { if_expression }
            block_node = None
            for child in else_clause.children:
                if child.type == 'block':
                    block_node = child
                    break

            if not block_node:
                continue

            # Check what's in the block
            # Block can contain: { expression_statement(if_expression) ... } or { if_expression ... }
            block_statements = [
                c for c in block_node.children
                if c.type in ('if_expression', 'expression_statement') and c.type != 'comment'
            ]

            # Should have exactly 1 statement
            if len(block_statements) != 1:
                continue

            stmt = block_statements[0]

            # Check if it's directly an if_expression or wrapped in expression_statement
            if stmt.type == 'if_expression':
                results.append((
                    else_clause.start_byte,
                    block_node.start_byte + 1,  # Include opening brace
                    "Collapse else-if: remove braces around single if"
                ))
            elif stmt.type == 'expression_statement':
                # Check if the expression_statement contains an if_expression
                for child in stmt.children:
                    if child.type == 'if_expression':
                        results.append((
                            else_clause.start_byte,
                            block_node.start_byte + 1,  # Include opening brace
                            "Collapse else-if: remove braces around single if"
                        ))
                        break

        return results

    # ========== Main Methods ==========

    def find(self, rust_code: str) -> List[Tuple[int, int, str]]:
        """Find all three types of clippy pattern violations."""
        results = []

        # Find needless returns
        results.extend(self._find_needless_returns(rust_code))

        # Find assign ops
        results.extend(self._find_assign_ops(rust_code))

        # Find collapsible else if
        results.extend(self._find_collapsible_else_if(rust_code))

        return results

    def fix(self, rust_code: str) -> str:
        """Apply fixes for all three patterns iteratively until idempotent."""
        code = rust_code

        # Keep applying fixes until no more matches are found
        # This ensures idempotency: after fixing, we check if new patterns appear
        max_iterations = 100
        for iteration in range(max_iterations):
            original_code = code

            # Check if there are any matches
            matches = self.find(code)
            if not matches:
                # No matches found - we've reached fixed point (idempotent)
                break

            # Apply all three fix methods in sequence
            code = self._fix_needless_returns(code)
            code = self._fix_assign_ops(code)
            code = self._fix_collapsible_else_if(code)

            # If no changes after applying fixes, we're done
            if code == original_code:
                break

        return code

    def _fix_needless_returns(self, rust_code: str) -> str:
        """Remove needless return statements."""
        try:
            root = self.parse(rust_code)
        except Exception:
            # If parsing fails, return original code
            return rust_code

        replacements = []
        return_nodes = self.find_nodes(root, 'return_expression')

        for return_node in return_nodes:
            if not self._is_return_statement(return_node, rust_code):
                continue

            stmt_parent = self.get_statement_node(return_node)
            if not stmt_parent:
                continue

            if self._is_last_statement_in_block(return_node, rust_code):
                expr_text = self._extract_return_expr(return_node, rust_code)
                if not expr_text:
                    continue

                # IMPORTANT: Only replace if statement_parent is expression_statement
                # This ensures we're replacing "return expr;" not some larger block
                if stmt_parent.type != 'expression_statement':
                    continue

                # Validate byte offsets before adding to replacements
                start = stmt_parent.start_byte
                end = stmt_parent.end_byte

                # Verify the byte range is valid
                if start < 0 or end > len(rust_code) or start >= end:
                    continue

                # Verify the code at this position starts with 'return'
                snippet = rust_code[start:start+20]
                if not snippet.strip().startswith('return '):
                    continue

                # Replace the entire statement "return expr;" with just "expr" (no semicolon)
                replacements.append((start, end, expr_text))

        return self.apply_replacements(rust_code, replacements)

    def _fix_assign_ops(self, rust_code: str) -> str:
        """Convert assignments to compound assignment operators."""
        root = self.parse(rust_code)
        replacements = []

        assignments = self.find_nodes_by_predicate(
            root,
            lambda n: n.type == 'assignment_expression'
        )

        for assign_node in assignments:
            left_node = None
            right_node = None

            for child in assign_node.children:
                if left_node is None and child.type != '=':
                    left_node = child
                elif child.type == '=' and right_node is None:
                    continue
                elif right_node is None and left_node is not None:
                    right_node = child
                    break

            if not (left_node and right_node) or right_node.type != 'binary_expression':
                continue

            bin_children = [c for c in right_node.children if c.type != 'comment']
            if len(bin_children) < 3:
                continue

            bin_left = bin_children[0]
            bin_op = bin_children[1]
            bin_right = bin_children[2]

            op_text = self.node_text(bin_op, rust_code).strip()
            left_text = self.node_text(left_node, rust_code).strip()
            bin_left_text = self.node_text(bin_left, rust_code).strip()
            bin_right_text = self.node_text(bin_right, rust_code).strip()

            # Validate texts are non-empty before creating replacement
            if not left_text or not op_text or not bin_right_text:
                continue

            if self._are_expressions_equal(left_text, bin_left_text):
                if op_text in ('+', '-', '*', '/', '%', '&', '|', '^', '<<', '>>'):
                    # Create the compound assignment
                    compound = f"{left_text} {op_text}= {bin_right_text}"
                    replacements.append((
                        assign_node.start_byte,
                        assign_node.end_byte,
                        compound
                    ))

        return self.apply_replacements(rust_code, replacements)

    def _fix_collapsible_else_if(self, rust_code: str) -> str:
        """Collapse else { if ... } to else if ...."""
        # Use tree-sitter to safely parse and transform
        root = self.parse(rust_code)

        # Find all if-expressions that have an else_clause with just a block containing an if
        if_nodes = self.find_nodes(root, 'if_expression')

        replacements = []

        for if_node in if_nodes:
            # Look for else_clause
            else_clause = None
            for child in if_node.children:
                if child.type == 'else_clause':
                    else_clause = child
                    break

            if not else_clause:
                continue

            # Check if else_clause structure is: else block where block contains only if_expression
            block = None
            for child in else_clause.children:
                if child.type == 'block':
                    block = child
                    break

            if not block:
                # else if case - already in good form
                continue

            # Check what's inside the block
            # The block children should be: {, [expression_statement containing if_expression], }
            non_comment_children = [c for c in block.children if c.type not in ('comment', '{', '}')]

            if len(non_comment_children) != 1:
                continue

            stmt = non_comment_children[0]

            # Check if it's an if_expression directly or wrapped in expression_statement
            if_expr = None
            if stmt.type == 'if_expression':
                if_expr = stmt
            elif stmt.type == 'expression_statement':
                # Check if the expression_statement contains an if_expression
                for child in stmt.children:
                    if child.type == 'if_expression':
                        if_expr = child
                        break

            if not if_expr:
                continue

            # Safe to transform! Record this for replacement
            # Find the "if" keyword start position in the if_expression
            if_keyword_pos = if_expr.start_byte

            # Replace "else {" with "else if" and remove the closing }
            # The replacement should be: "else if" + everything from "if" to "}"
            else_block_start = else_clause.start_byte
            block_end = block.end_byte

            # Get the if keyword and the condition+body
            if_content = rust_code[if_keyword_pos:block_end-1]  # Exclude the closing }

            # The content after the if keyword should be " <condition> { <body> } [else ...]"
            # We need to remove the leading whitespace and newlines before constructing "else if"
            if_part = if_content[2:].lstrip()  # Skip "if" and leading whitespace

            replacements.append((else_block_start, block_end, f"else if {if_part}"))

        # Apply replacements in reverse order to maintain byte positions
        result = rust_code
        for start, end, replacement in reversed(replacements):
            result = result[:start] + replacement + result[end:]

        return result
