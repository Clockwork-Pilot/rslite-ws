import os
import re
import sys
from pathlib import Path
from tree_sitter import Language, Parser
import tree_sitter_rust

parser = Parser()
parser.language = Language(tree_sitter_rust.language())

# -------------------------------
# AST helpers
# -------------------------------
SUPPORTED_NODE_TYPES = {"struct_item", "type_item", "enum_item"}


def find_item_nodes(source_code, item_name):
    tree = parser.parse(bytes(source_code, "utf8"))
    root = tree.root_node
    results = []

    def walk(node):
        if node.type in SUPPORTED_NODE_TYPES:
            name_node = node.child_by_field_name("name")
            if name_node:
                name = source_code[name_node.start_byte:name_node.end_byte]
                if name == item_name:
                    results.append(node)
        for child in node.children:
            walk(child)

    walk(root)
    return results


def find_all_top_level_items(source_code):
    """Return dict: name -> node for all top-level supported items."""
    tree = parser.parse(bytes(source_code, "utf8"))
    root = tree.root_node
    result = {}

    def walk(node):
        if node.type in SUPPORTED_NODE_TYPES:
            name_node = node.child_by_field_name("name")
            if name_node:
                name = source_code[name_node.start_byte:name_node.end_byte]
                result[name] = node
        for child in node.children:
            walk(child)

    walk(root)
    return result


def find_type_refs_in_node(source_code, node):
    """Collect all type_identifier names referenced within a node's subtree."""
    refs = set()

    def walk(n):
        if n.type == "type_identifier":
            refs.add(source_code[n.start_byte:n.end_byte])
        for child in n.children:
            walk(child)

    walk(node)
    return refs


def extract_item_with_attributes(source_code, node):
    """Return (text, start_byte, end_byte) including any preceding attribute_item nodes."""
    start = node.start_byte
    end = node.end_byte

    prev = node.prev_named_sibling
    while prev and prev.type == "attribute_item":
        start = prev.start_byte
        prev = prev.prev_named_sibling

    return source_code[start:end], start, end


def is_name_referenced(source_code, name):
    """Check if name appears as a whole word anywhere in source_code."""
    return bool(re.search(r'\b' + re.escape(name) + r'\b', source_code))


def collect_local_deps(source_code, item_node, top_level_items, visited=None):
    """
    Recursively find all items defined in this file that are referenced by item_node.
    Returns dict: name -> node (only items defined in this file).
    """
    if visited is None:
        visited = set()

    refs = find_type_refs_in_node(source_code, item_node)
    result = {}

    for ref in refs:
        if ref in visited:
            continue
        visited.add(ref)

        if ref in top_level_items:
            dep_node = top_level_items[ref]
            result[ref] = dep_node
            # Recurse into this dep's own dependencies
            sub = collect_local_deps(source_code, dep_node, top_level_items, visited)
            result.update(sub)

    return result


# -------------------------------
# Use statement handling
# -------------------------------
def ensure_use_statement(source_code, item_name, module_path):
    use_stmt = f"use {module_path}::{item_name};"

    if use_stmt in source_code:
        return source_code

    lines = source_code.splitlines()
    insert_idx = 0

    for i, line in enumerate(lines):
        if line.strip().startswith("use "):
            insert_idx = i + 1

    lines.insert(insert_idx, use_stmt)
    return "\n".join(lines) + "\n"


# -------------------------------
# Module path resolution
# -------------------------------
def module_path_from_file(file_path, base_dir):
    rel = Path(file_path).relative_to(base_dir)
    parts = list(rel.with_suffix("").parts)

    if parts[-1] == "mod":
        parts = parts[:-1]

    return "::".join(parts)


# -------------------------------
# File processing
# -------------------------------
def process_file(file_path, item_name, dest_module_path):
    with open(file_path, "r", encoding="utf8") as f:
        source = f.read()

    nodes = find_item_nodes(source, item_name)
    if not nodes:
        return None, source

    main_node = nodes[0]
    top_level = find_all_top_level_items(source)

    # Collect all local deps recursively
    dep_nodes = collect_local_deps(source, main_node, top_level)

    # Extract spans for main item and all deps
    main_text, main_start, main_end = extract_item_with_attributes(source, main_node)

    dep_spans = {}  # name -> (text, start, end)
    for name, node in dep_nodes.items():
        dep_spans[name] = extract_item_with_attributes(source, node)

    # Build a source with ALL candidates removed to check residual references
    all_spans = [(main_start, main_end)] + [(s, e) for _, s, e in dep_spans.values()]
    stripped = source
    for start, end in sorted(all_spans, key=lambda x: x[0], reverse=True):
        stripped = stripped[:start] + stripped[end:]

    # Deps still referenced in stripped source stay in the source file
    deps_to_keep = {name for name in dep_nodes if is_name_referenced(stripped, name)}

    # Remove main item and unreferenced deps from source
    spans_to_remove = [(main_start, main_end)]
    for name, (_, start, end) in dep_spans.items():
        if name not in deps_to_keep:
            spans_to_remove.append((start, end))

    new_source = source
    for start, end in sorted(spans_to_remove, key=lambda x: x[0], reverse=True):
        new_source = new_source[:start] + new_source[end:]

    # Add use statement for main item (it's been removed from this file)
    new_source = ensure_use_statement(new_source, item_name, dest_module_path)

    # Collect everything that goes to destination: main + all deps (kept or not)
    extracted = {item_name: main_text}
    for name, (text, _, _) in dep_spans.items():
        extracted[name] = text

    if deps_to_keep:
        print(f"  Kept in source (still referenced): {', '.join(sorted(deps_to_keep))}")

    return extracted, new_source


# -------------------------------
# Main logic
# -------------------------------
def main():
    if len(sys.argv) < 4:
        print("Usage:")
        print("  python deduplicate_struct.py <ItemName> <dest_file> <search_dir> [<search_dir>...]")
        sys.exit(1)

    item_name = sys.argv[1]
    dest_file = Path(sys.argv[2]).resolve()
    search_dirs = [Path(p).resolve() for p in sys.argv[3:]]

    base_dir = Path(os.path.commonpath(search_dirs))
    dest_module_path = module_path_from_file(dest_file, base_dir)

    # name -> list of text variants collected across all source files
    all_collected = {}

    for search_dir in search_dirs:
        for path in sorted(search_dir.rglob("*.rs")):
            extracted, new_source = process_file(path, item_name, dest_module_path)

            if extracted:
                for name, text in extracted.items():
                    all_collected.setdefault(name, []).append(text.strip())

                with open(path, "w", encoding="utf8") as f:
                    f.write(new_source)

                print(f"Updated: {path}")

    if not all_collected:
        print("No definitions found.")
        return

    # Per-name uniqueness check
    canonical_defs = {}  # name -> canonical text
    for name, texts in all_collected.items():
        unique = list(dict.fromkeys(texts))
        if len(unique) > 1:
            print(f"ERROR: Multiple different definitions for '{name}'!\n")
            for i, d in enumerate(unique):
                print(f"--- {name} definition {i+1} ---\n{d}\n")
            sys.exit(1)
        canonical_defs[name] = unique[0]

    # Write to destination: deps first (topological order approximation), main last
    dest_file.parent.mkdir(parents=True, exist_ok=True)
    existing = dest_file.read_text(encoding="utf8") if dest_file.exists() else ""

    # Write deps before main item
    write_order = [n for n in canonical_defs if n != item_name] + [item_name]

    with open(dest_file, "a", encoding="utf8") as f:
        for name in write_order:
            text = canonical_defs[name]
            if re.search(r'\b' + re.escape(name) + r'\b', existing):
                print(f"Already exists in destination: {name}")
            else:
                f.write("\n\n// --- Deduplicated ---\n\n")
                f.write(text)
                f.write("\n")
                existing += text  # update so subsequent names see it
                label = "moved" if name == item_name else "copied (dep)"
                print(f"  {label}: '{name}' → {dest_file}")


if __name__ == "__main__":
    main()
