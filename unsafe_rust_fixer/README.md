# Unsafe Rust Pattern Fixer - Plugin System

This directory contains the pluggable unsafe pattern detection and fixing framework for Rust code.

## Architecture

### Core Components

1. **`base.py`** - Base class for all plugins
   - Provides tree-sitter integration
   - Implements common AST utilities
   - Defines the plugin interface (abstract methods)

2. **Plugin Files** - Individual pattern implementations
   - Each file implements one or more `UnsafePatternPlugin` subclasses
   - Automatically discovered and loaded at runtime
   - Pattern names derived from class names (snake_case)

## Execution Model

### Pattern priority — order of application

Patterns are applied **sequentially by priority (descending)** across the project. Higher priority runs first:

```
priority 14  raw_ptr_deref_field_chain        ← applied first
priority 10  zero_init_with_memset
priority  8  simplify_constant_offset_cast
priority  0  (default — everything else)      ← applied last
```

Each pattern fully completes across **all files** before the next pattern starts. This guarantees that pattern N+1 always sees the output of pattern N.

Set priority by overriding the `priority` property (default is `0`):

```python
@property
def priority(self) -> int:
    return 10  # runs before default-priority patterns
```

Use higher priority when your pattern produces output that another pattern depends on, or when it makes structural changes that lower-priority patterns should see.

### File parallelism

Within each pattern, files are processed **in parallel** (multiprocessing pool). Files are independent — no pattern reads output written to a different file by the same pattern run.

```
pattern (priority 14):  [file_a, file_b, file_c, ...]  ← parallel
pattern (priority 10):  [file_a, file_b, file_c, ...]  ← parallel
...
```

## Creating New Plugins

### Basic Structure

```python
"""Plugin: Describe what this pattern detects."""
from typing import List, Tuple
from .base import UnsafePatternPlugin


class MyPatternPlugin(UnsafePatternPlugin):
    """Find and report my unsafe pattern."""

    @property
    def name(self) -> str:
        return "my_pattern"

    @property
    def description(self) -> str:
        return "Human-readable description of the pattern"

    def find(self, rust_code: str) -> List[Tuple[int, int, str]]:
        """Find all occurrences of this pattern.

        Returns list of (start_byte, end_byte, description) tuples.
        """
        results = []
        root = self.parse(rust_code)

        # Use tree-sitter utilities to find patterns
        nodes = self.find_nodes(root, 'some_node_type')
        for node in nodes:
            if is_my_pattern(node):
                line = self.node_line(node, rust_code)
                text = self.node_text(node, rust_code)
                results.append((
                    node.start_byte,
                    node.end_byte,
                    f"Line {line}: Found pattern: {text}"
                ))

        return results

    def fix(self, rust_code: str) -> str:
        """Apply fixes to the code.

        Returns the fixed code, or original if not fixable.
        If fixing is not safe/possible, just return the original code.
        """
        matches = self.find(rust_code)

        if not matches:
            return rust_code

        # Process matches in reverse order to maintain byte offsets
        for start, end, _ in sorted(matches, reverse=True):
            old_text = rust_code[start:end]
            new_text = fix_pattern(old_text)
            rust_code = rust_code[:start] + new_text + rust_code[end:]

        return rust_code
```

## Available Utilities (from base.py)

### Parsing & Tree Navigation
- `parse(code: str)` - Parse Rust code, return root AST node
- `find_nodes(node, node_type)` - Find all nodes of a type
- `find_nodes_by_predicate(node, predicate)` - Find nodes matching a condition
- `get_parent_of_type(node, node_type)` - Find parent of specific type

### Node Information
- `node_text(node, code)` - Extract text of a node
- `node_line(node, code)` - Get line number of a node
- `has_unsafe_block(node)` - Check if node is in unsafe block

### Code Transformation
- `replace_node(code, node, replacement)` - Replace node's text

## Tree-Sitter Rust Node Types

Common node types for Rust patterns:
- `unsafe_block` - `unsafe { ... }` blocks
- `function_item` - Function definitions
- `unary_expression` - Unary operations (*, !, -, +, &, etc.)
- `binary_expression` - Binary operations
- `call_expression` - Function calls
- `cast_expression` - Type casts (as)
- `pointer_type` - Raw pointer types
- `field_expression` - Field access (obj.field)
- `index_expression` - Array indexing

## Plugin Loading

Plugins are automatically discovered by the main engine:
1. The engine scans the `unsafe_fixer_plugins/` directory
2. For each `.py` file (except `__init__.py` and `base.py`):
   - Loads the module
   - Finds classes that inherit from `UnsafePatternPlugin`
   - Instantiates them and registers by `name`

Plugin class names should be in PascalCase with "Plugin" suffix:
- `UnsafeDerefRawPtrPlugin` → name: `unsafe_deref_raw_ptr`
- `MallocUnsafePlugin` → name: `malloc_unsafe`

## Built-in Plugins

### `unsafe_deref_raw_ptr`
Detects raw pointer dereferences (`*ptr`) that are not in unsafe blocks.

### `unsafe_cast`
Detects casts to raw pointers using `as` operator.
Note: These require manual review, automatic fixes are not applied.

### `unsafe_fn_call`
Detects calls to common unsafe functions (std::ptr::*, std::mem::*, libc::*, etc.)
outside of unsafe blocks.

## Testing Your Plugin

```bash
# List available patterns
python3 unsafe-rust-fixer.py --list-patterns

# Scan a file
python3 unsafe-rust-fixer.py path/to/rust/file.rs

# Apply fixes
python3 unsafe-rust-fixer.py path/to/rust/file.rs --fix unsafe_deref_raw_ptr

# Verbose mode
python3 unsafe-rust-fixer.py path/to/rust/file.rs -v
```

## Dependencies

- `tree-sitter` - Python bindings for tree-sitter parser
- `tree-sitter-rust` - Rust language grammar for tree-sitter

Install with:
```bash
pip install tree-sitter tree-sitter-rust
```

## Pattern Naming Conventions

- Use lowercase with underscores: `unsafe_pattern_name`
- Be descriptive but concise
- Avoid redundant "unsafe" prefix if it's implied (it is for this system)
- Examples: `deref_raw_ptr`, `unchecked_cast`, `ffi_call`

## Notes on Tree-Sitter

Tree-sitter provides excellent performance and can parse incomplete/syntactically invalid Rust code. This makes it ideal for linting tools that need to handle work-in-progress code.

When working with AST nodes:
- All byte offsets are UTF-8 byte positions, not character positions
- `start_point` and `end_point` are (row, column) tuples (0-indexed)
- Parent traversal goes up: `node.parent` → root
- Child traversal goes down: `node.children` → leaf nodes
