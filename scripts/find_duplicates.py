import json
import sys
from pathlib import Path
from tree_sitter import Language, Parser
import tree_sitter_rust

parser = Parser()
parser.language = Language(tree_sitter_rust.language())

SUPPORTED_NODE_TYPES = {"struct_item", "type_item", "enum_item"}


def collect_items(file_path):
    """Return list of (name, kind) for all top-level supported items in file."""
    try:
        source = Path(file_path).read_text(encoding="utf8")
    except OSError:
        return []

    tree = parser.parse(bytes(source, "utf8"))
    results = []

    def walk(node):
        if node.type in SUPPORTED_NODE_TYPES:
            name_node = node.child_by_field_name("name")
            if name_node:
                name = source[name_node.start_byte:name_node.end_byte]
                results.append((name, node.type))
        for child in node.children:
            walk(child)

    walk(tree.root_node)
    return results


def main():
    if len(sys.argv) < 2:
        print("Usage: python find_duplicates.py <search_dir> [<search_dir>...]")
        sys.exit(1)

    search_dirs = [Path(p).resolve() for p in sys.argv[1:]]

    # name -> {kind, files: []}
    index = {}

    for search_dir in search_dirs:
        for path in sorted(search_dir.rglob("*.rs")):
            rel = str(path.relative_to(search_dir))
            for name, kind in collect_items(path):
                if name not in index:
                    index[name] = {"kind": kind, "files": []}
                if rel not in index[name]["files"]:
                    index[name]["files"].append(rel)

    # Build output structure sorted by occurrence count ascending
    names = {}
    for name, info in sorted(index.items(), key=lambda x: len(x[1]["files"])):
        names[name] = {
            "kind": info["kind"],
            "occurs": [
                {"count": i + 1, "file": f}
                for i, f in enumerate(info["files"])
            ],
        }

    print(json.dumps({"names": names}, indent=2))


if __name__ == "__main__":
    main()
