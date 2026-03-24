"""Base class for unsafe pattern plugins."""
from abc import ABC, abstractmethod
from typing import List, Tuple, Optional, Callable, Any

try:
    from tree_sitter import Language, Parser
    from tree_sitter_rust import language as rust_language_fn
    TREE_SITTER_AVAILABLE = True
except ImportError:
    TREE_SITTER_AVAILABLE = False
    Language = object  # type: ignore
    Parser = object  # type: ignore
    rust_language_fn = None  # type: ignore


class UnsafePatternPlugin(ABC):
    """Base class for all unsafe pattern plugins.

    Plugins should inherit from this class and implement find() and fix() methods.
    """

    def __init__(self) -> None:
        """Initialize the plugin with tree-sitter Rust parser."""
        if not TREE_SITTER_AVAILABLE:
            raise ImportError(
                "tree-sitter and tree-sitter-rust are required. "
                "Install with: pip install tree-sitter tree-sitter-rust"
            )
        self.language = Language(rust_language_fn())
        self.parser: Parser = Parser()
        self.parser.language = self.language

    @property
    @abstractmethod
    def name(self) -> str:
        """Return the name of the pattern (e.g., 'unsafe_pointer_deref')."""
        pass

    @property
    @abstractmethod
    def description(self) -> str:
        """Return a human-readable description of the pattern."""
        pass

    @property
    def priority(self) -> int:
        """Return the priority of this pattern (higher = more important).

        Default: 0 (low priority). Override in subclasses for higher priority.
        """
        return 0

    @abstractmethod
    def find(self, rust_code: str) -> List[Tuple[int, int, str]]:
        """Find all occurrences of this unsafe pattern.

        Args:
            rust_code: The Rust source code to search

        Returns:
            List of (start_byte, end_byte, description) tuples for each match
        """
        pass

    @abstractmethod
    def fix(self, rust_code: str) -> str:
        """Apply fixes for this pattern.

        Args:
            rust_code: The Rust source code to fix

        Returns:
            Fixed Rust code (or original if not fixable)
        """
        pass

    def test(self) -> bool:
        """Run self-contained tests for this pattern.

        Returns:
            True if all tests pass, False otherwise
        """
        return True  # Default: no tests

    # ========== Utility Methods ==========

    def parse(self, code: str) -> Any:
        """Parse Rust code and return the AST root node."""
        tree = self.parser.parse(code.encode('utf-8'))
        return tree.root_node

    def find_nodes(self, node: Any, node_type: str) -> List[Any]:
        """Find all nodes of a specific type in the tree."""
        results: List[Any] = []

        def traverse(n: Any) -> None:
            if n.type == node_type:
                results.append(n)
            for child in n.children:
                traverse(child)

        traverse(node)
        return results

    def find_nodes_by_predicate(
        self, node: Any, predicate: Callable[[Any], bool]
    ) -> List[Any]:
        """Find all nodes matching a predicate function."""
        results: List[Any] = []

        def traverse(n: Any) -> None:
            if predicate(n):
                results.append(n)
            for child in n.children:
                traverse(child)

        traverse(node)
        return results

    def node_text(self, node: Any, code: str) -> str:
        """Extract the text of a node from the source code."""
        start = node.start_byte
        end = node.end_byte
        return code[start:end]

    def node_line(self, node: Any, code: str) -> int:
        """Get the line number of a node."""
        return node.start_point[0] + 1

    def replace_node(self, code: str, node: Any, replacement: str) -> str:
        """Replace a node's text with replacement text."""
        start = node.start_byte
        end = node.end_byte
        return code[:start] + replacement + code[end:]

    def get_parent_of_type(self, node: Any, node_type: str) -> Optional[Any]:
        """Find the parent of a specific type."""
        current = node.parent
        while current:
            if current.type == node_type:
                return current
            current = current.parent
        return None

    def has_unsafe_block(self, node: Any) -> bool:
        """Check if a node is inside an unsafe block."""
        current = node.parent
        while current:
            if current.type == 'unsafe_block':
                return True
            current = current.parent
        return False

    def is_zero_value_node(self, node: Any, code: str) -> bool:
        """Check if a node evaluates to the integer literal zero (possibly cast).

        Handles patterns like:
          - 0
          - 0 as c_int
          - 0 as c_int as isize

        Note: this grammar uses 'type_cast_expression' (not 'cast_expression')
        for  expr as Type  forms.
        """
        if node.type == 'integer_literal':
            return self.node_text(node, code).strip() == '0'
        # Both names seen across tree-sitter-rust versions
        elif node.type in ('cast_expression', 'type_cast_expression'):
            inner = node.child_by_field_name('value')
            if inner is not None:
                return self.is_zero_value_node(inner, code)
        return False

    def get_statement_node(self, node: Any) -> Optional[Any]:
        """Walk up the tree to find the enclosing statement node.

        Returns the nearest expression_statement or let_declaration ancestor.
        """
        current = node
        while current:
            if current.type in ('expression_statement', 'let_declaration'):
                return current
            current = current.parent
        return None

    def apply_replacements(
        self, code: str, replacements: List[Tuple[int, int, str]]
    ) -> str:
        """Apply multiple (start_byte, end_byte, replacement) edits to code.

        Applies in reverse start-byte order so earlier replacements don't
        shift the byte offsets of later ones.
        """
        for start, end, replacement in sorted(
            replacements, key=lambda x: x[0], reverse=True
        ):
            code = code[:start] + replacement + code[end:]
        return code
