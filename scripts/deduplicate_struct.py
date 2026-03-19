import os
import sys
from pathlib import Path
from tree_sitter import Language, Parser
import tree_sitter_rust

# -------------------------------
# Tree-sitter setup (auto-build)
# -------------------------------
BUILD_DIR = "build"
LIB_NAME = "rust.so" if os.name != "nt" else "rust.dll"
LIB_PATH = os.path.join(BUILD_DIR, LIB_NAME)
RUST_GRAMMAR_PATH = "tree-sitter-rust"

# if not os.path.exists(LIB_PATH):
#     os.makedirs(BUILD_DIR, exist_ok=True)
#     print("Building tree-sitter Rust parser...")
#     Language.build_library(
#         LIB_PATH,
#         [RUST_GRAMMAR_PATH],
#     )

parser = Parser()
parser.language = Language(tree_sitter_rust.language())

# -------------------------------
# AST helpers
# -------------------------------
def find_struct_nodes(source_code, struct_name):
    tree = parser.parse(bytes(source_code, "utf8"))
    root = tree.root_node

    results = []

    def walk(node):
        if node.type == "struct_item":
            name_node = node.child_by_field_name("name")
            if name_node:
                name = source_code[name_node.start_byte:name_node.end_byte]
                if name == struct_name:
                    results.append(node)
        for child in node.children:
            walk(child)

    walk(root)
    return results


# def extract_node_text(source_code, node):
#     return source_code[node.start_byte:node.end_byte]

def extract_struct_with_attributes(source_code, node):
    start = node.start_byte
    end = node.end_byte

    # Walk backwards to include attributes
    prev = node.prev_named_sibling

    while prev and prev.type == "attribute_item":
        start = prev.start_byte
        prev = prev.prev_named_sibling

    return source_code[start:end], start, end

def remove_nodes(source_code, nodes):
    nodes = sorted(nodes, key=lambda n: n.start_byte, reverse=True)
    for node in nodes:
        source_code = source_code[:node.start_byte] + source_code[node.end_byte:]
    return source_code


# -------------------------------
# Use statement handling
# -------------------------------
def ensure_use_statement(source_code, struct_name, module_path):
    use_stmt = f"use {module_path}::{struct_name};"

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

    # Handle mod.rs → parent module
    if parts[-1] == "mod":
        parts = parts[:-1]

    return "::".join(parts)


# -------------------------------
# File processing
# -------------------------------
def process_file(file_path, struct_name, dest_module_path):
    with open(file_path, "r", encoding="utf8") as f:
        source = f.read()

    nodes = find_struct_nodes(source, struct_name)

    if not nodes:
        return None, source

    spans = [extract_struct_with_attributes(source, n) for n in nodes]

    extracted = [s[0] for s in spans]

    # Remove using spans instead of raw nodes
    new_source = source
    for _, start, end in sorted(spans, key=lambda x: x[1], reverse=True):
        new_source = new_source[:start] + new_source[end:]

    new_source = ensure_use_statement(new_source, struct_name, dest_module_path)

    return extracted, new_source


# -------------------------------
# Main logic
# -------------------------------
def main():
    if len(sys.argv) < 4:
        print("Usage:")
        print("python deduplicate_struct.py <StructName> <dest_file> <search_dir> [<search_dir>...]")
        sys.exit(1)

    struct_name = sys.argv[1]
    dest_file = Path(sys.argv[2]).resolve()
    search_dirs = [Path(p).resolve() for p in sys.argv[3:]]

    base_dir = Path(os.path.commonpath(search_dirs))

    collected_defs = []

    dest_module_path = module_path_from_file(dest_file, base_dir)

    for search_dir in search_dirs:
        for path in search_dir.rglob("*.rs"):
            extracted, new_source = process_file(
                path,
                struct_name,
                dest_module_path,
            )

            if extracted:
                collected_defs.extend(extracted)

                with open(path, "w", encoding="utf8") as f:
                    f.write(new_source)

                print(f"Updated: {path}")

    if not collected_defs:
        print("No struct definitions found.")
        return

    # -------------------------------
    # Safety: ensure all definitions identical
    # -------------------------------
    normalized = [d.strip() for d in collected_defs]
    unique_defs = list(dict.fromkeys(normalized))

    if len(unique_defs) > 1:
        print("ERROR: Found multiple DIFFERENT struct definitions!\n")
        for i, d in enumerate(unique_defs):
            print(f"--- Definition {i+1} ---\n{d}\n")
        sys.exit(1)

    canonical_def = unique_defs[0]

    # -------------------------------
    # Write destination
    # -------------------------------
    dest_file.parent.mkdir(parents=True, exist_ok=True)

    existing = ""
    if dest_file.exists():
        existing = dest_file.read_text(encoding="utf8")

    if struct_name in existing:
        print(f"Struct already exists in destination: {dest_file}")
    else:
        with open(dest_file, "a", encoding="utf8") as f:
            f.write("\n\n// --- Deduplicated Struct ---\n\n")
            f.write(canonical_def)
            f.write("\n")

        print(f"\nStruct '{struct_name}' moved to {dest_file}")


if __name__ == "__main__":
    main()
