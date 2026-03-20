import sys
from pathlib import Path
from deduplicate_struct import (
    find_item_nodes,
    extract_item_with_attributes,
    find_foreign_opaque_types,
    ensure_use_statement,
    find_crate_root,
    cleanup_redundant_use_stmts,
)


def crate_module_path(file_path):
    """Compute module path relative to the crate root (directory containing Cargo.toml)."""
    crate_root = find_crate_root(file_path)
    if crate_root is None:
        raise RuntimeError(f"No Cargo.toml found above {file_path}")
    rel = Path(file_path).resolve().relative_to(crate_root).with_suffix("")
    parts = list(rel.parts)
    if parts and parts[-1] == "mod":
        parts = parts[:-1]
    return "::".join(parts)


def pick_canonical(candidates):
    """Pick the canonical file: prefer files with exactly 1 definition (the 'native' file)."""
    single_occ = [(p, nodes) for p, nodes in candidates if len(nodes) == 1]
    if len(single_occ) == 1:
        return single_occ[0][0]
    if single_occ:
        chosen = single_occ[0][0]
        print(f"  Warning: {len(single_occ)} single-occurrence files, picking {chosen}")
        return chosen
    chosen = candidates[0][0]
    print(f"  Warning: no single-occurrence file, picking {chosen}")
    return chosen


def remove_item_spans(source, nodes):
    """Remove item definitions (with attributes) from source, return new source and removed spans."""
    spans = [extract_item_with_attributes(source, n) for n in nodes]
    new_source = source
    for _, start, end in sorted(spans, key=lambda x: x[1], reverse=True):
        new_source = new_source[:start] + new_source[end:]
    return new_source


def process_non_canonical(file_path, item_name, canonical_module):
    """Remove duplicate definition from a non-canonical file and add use crate::... import."""
    source = file_path.read_text(encoding="utf8")
    nodes = find_item_nodes(source, item_name)
    if not nodes:
        return False

    new_source = remove_item_spans(source, nodes)

    # Also remove extern opaque type declarations for this item.
    # Re-parse the already-modified source since byte offsets shifted.
    extern_opaque_new = find_foreign_opaque_types(new_source)
    if item_name in extern_opaque_new:
        info = extern_opaque_new[item_name]
        new_source = new_source[:info["start"]] + new_source[info["end"]:]

    # Handle local deps: collect deps of the removed item that are no longer
    # referenced in the rest of the file. These are candidates for removal too,
    # but only if they would also be available from the canonical module.
    # For safety, we leave deps alone — they'll be handled in their own dedup pass.

    new_source = ensure_use_statement(new_source, item_name, "crate", canonical_module)

    file_path.write_text(new_source, encoding="utf8")
    cleanup_redundant_use_stmts(file_path)
    return True


def main():
    if len(sys.argv) < 3:
        print("Usage: python inline_dedup.py <ItemName> <search_dir> [<search_dir>...]")
        sys.exit(1)

    item_name = sys.argv[1]
    search_dirs = [Path(p).resolve() for p in sys.argv[2:]]

    # Find all files containing the definition
    candidates = []
    for search_dir in search_dirs:
        for path in sorted(search_dir.rglob("*.rs")):
            try:
                source = path.read_text(encoding="utf8")
            except OSError:
                continue
            nodes = find_item_nodes(source, item_name)
            if nodes:
                candidates.append((path, nodes))

    if len(candidates) < 2:
        print(f"Found {len(candidates)} file(s) with '{item_name}' — nothing to deduplicate.")
        return

    # Pick canonical file
    canonical_path = pick_canonical(candidates)
    canonical_module = crate_module_path(canonical_path)

    # Verify all definitions match
    canonical_source = canonical_path.read_text(encoding="utf8")
    canonical_nodes = find_item_nodes(canonical_source, item_name)
    canonical_text = extract_item_with_attributes(canonical_source, canonical_nodes[0])[0].strip()

    print(f"Canonical: {canonical_path}")
    print(f"  module path: {canonical_module}")
    print(f"  duplicates in {len(candidates) - 1} other file(s)")

    mismatches = []
    for path, nodes in candidates:
        if path == canonical_path:
            continue
        source = path.read_text(encoding="utf8")
        for node in find_item_nodes(source, item_name):
            text = extract_item_with_attributes(source, node)[0].strip()
            if text != canonical_text:
                mismatches.append((path, text))

    if mismatches:
        print(f"\nERROR: {len(mismatches)} file(s) have different definitions for '{item_name}'!")
        print(f"\n--- canonical ({canonical_path}) ---")
        print(canonical_text)
        for path, text in mismatches:
            print(f"\n--- {path} ---")
            print(text)
        sys.exit(1)

    # Process non-canonical files
    for path, nodes in candidates:
        if path == canonical_path:
            continue
        if process_non_canonical(path, item_name, canonical_module):
            print(f"  Updated: {path}")

    print("Done.")


if __name__ == "__main__":
    main()
