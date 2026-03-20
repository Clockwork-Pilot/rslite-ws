import os
import re
import sys
from pathlib import Path
from tree_sitter import Language, Parser
import tree_sitter_rust
from crates_common import import_crate_name

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


def find_foreign_opaque_types(source_code):
    """Find opaque extern type declarations inside extern blocks (pub type Foo;).
    tree-sitter-rust parses these as ERROR nodes inside foreign_mod_item.
    Returns dict: name -> {"text": 'extern "C" { pub type Foo; }', "start": N, "end": N}.
    """
    tree = parser.parse(bytes(source_code, "utf8"))
    result = {}

    def check_error_node(item, next_sibling=None):
        """Check if an ERROR node represents a 'pub type Foo;' foreign opaque type."""
        has_type_kw = any(c.type == "type" for c in item.children)
        id_node = next((c for c in item.children if c.type == "identifier"), None)
        if not has_type_kw or not id_node:
            return
        name = source_code[id_node.start_byte:id_node.end_byte]
        start = item.start_byte
        end = item.end_byte
        # Include the semicolon (empty_statement) if separate
        if next_sibling and next_sibling.type == "empty_statement":
            end = next_sibling.end_byte
        # If the ERROR contains its own semicolon, the end already covers it
        # Include trailing newline in the span to avoid blank lines
        while end < len(source_code) and source_code[end] == "\n":
            end += 1
        decl = source_code[start:end].strip()
        result[name] = {
            "text": f'extern "C" {{\n    {decl}\n}}',
            "start": start,
            "end": end,
        }

    def walk(node):
        if node.type == "foreign_mod_item":
            for child in node.children:
                if child.type == "declaration_list":
                    children = list(child.children)
                    for i, item in enumerate(children):
                        next_sib = children[i + 1] if i + 1 < len(children) else None
                        if item.type == "ERROR":
                            check_error_node(item, next_sib)
                        # tree-sitter may group the last 'pub type Foo;' with a
                        # following 'fn ...' into a function_signature_item — check
                        # for ERROR children inside those too.
                        elif item.type == "function_signature_item":
                            for sub in item.children:
                                if sub.type == "ERROR":
                                    check_error_node(sub)
            return  # don't recurse into foreign_mod_item children
        for child in node.children:
            walk(child)

    walk(tree.root_node)
    return result


def find_use_stmts_for_names(source_code, names):
    """
    Find use_declaration texts in source_code that import any of the given names.
    Returns dict: name -> use statement text.
    """
    if not names:
        return {}

    tree = parser.parse(bytes(source_code, "utf8"))
    root = tree.root_node
    result = {}

    def walk(node):
        if node.type == "use_declaration":
            text = source_code[node.start_byte:node.end_byte]
            for name in names:
                if name not in result and re.search(r'\b' + re.escape(name) + r'\b', text):
                    result[name] = text
        for child in node.children:
            walk(child)

    walk(root)
    return result


# -------------------------------
# Use statement handling
# -------------------------------
def ensure_use_statement(source_code, item_name, use_prefix, module_path):
    if module_path:
        use_stmt = f"use {use_prefix}::{module_path}::{item_name};"
    else:
        use_stmt = f"use {use_prefix}::{item_name};"

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
    parts = list(Path(file_path).with_suffix("").parts)

    # Prefer path relative to the last 'src' directory (Rust crate convention)
    try:
        src_idx = len(parts) - 1 - list(reversed(parts)).index("src")
        parts = parts[src_idx + 1:]
    except ValueError:
        # Fallback: relative to base_dir
        parts = list(Path(file_path).relative_to(base_dir).with_suffix("").parts)

    if parts and parts[-1] in ("mod", "lib"):
        parts = parts[:-1]

    return "::".join(parts)


# -------------------------------
# File processing
# -------------------------------
def process_file(file_path, item_name, crate_name, dest_module_path, dest_crate_root):
    with open(file_path, "r", encoding="utf8") as f:
        source = f.read()

    nodes = find_item_nodes(source, item_name)
    if not nodes:
        return None, [], source

    main_node = nodes[0]
    top_level = find_all_top_level_items(source)

    # Collect all local deps recursively; exclude item_name to avoid self-reference
    # (the name field is a type_identifier and would otherwise add the item as its own dep)
    dep_nodes = collect_local_deps(source, main_node, top_level, visited={item_name})

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

    # Deps still referenced in stripped source need use statements (not local copies).
    # Keeping local copies alongside the dest version causes type conflicts.
    deps_still_referenced = {name for name in dep_nodes if is_name_referenced(stripped, name)}

    # Spans to remove from source (struct/type items) — byte offsets into original source.
    # Extern opaque type spans will be added below before applying all at once.
    spans_to_remove = [(main_start, main_end)] + [(s, e) for _, s, e in dep_spans.values()]

    # Use `crate::` if source and dest are in the same crate, else the external crate name.
    # Crate names in use statements use underscores (not hyphens).
    source_crate_root = find_crate_root(file_path)
    use_prefix = "crate" if source_crate_root == dest_crate_root else import_crate_name(crate_name)

    # Collect everything that goes to destination: main + all deps (kept or not)
    extracted = {item_name: main_text}
    for name, (text, _, _) in dep_spans.items():
        extracted[name] = text

    # Find use statements in source for external type refs (not locally defined).
    # Only carry a use statement to the destination if the imported name is actually
    # referenced in one of the items we're writing there.
    all_moved_nodes = [main_node] + list(dep_nodes.values())
    all_refs = set()
    for node in all_moved_nodes:
        all_refs |= find_type_refs_in_node(source, node)

    local_names = set(dep_nodes.keys()) | {item_name}
    external_refs = all_refs - local_names

    # Collect extern opaque types (pub type Foo; inside extern blocks) referenced
    # by the moved items. These must be MOVED (not copied) to avoid type incompatibility
    # across crates — both crates must share the same extern type instance.
    extern_opaque = find_foreign_opaque_types(source)
    extern_names_moved = set()
    for name in external_refs:
        if name in extern_opaque and name not in extracted:
            info = extern_opaque[name]
            extracted[name] = info["text"]
            extern_names_moved.add(name)

    # Re-apply ALL removals from original source (struct/type spans + extern opaque spans)
    # in one pass, since all byte offsets are relative to the original source.
    all_remove_spans = list(spans_to_remove)  # struct/type spans already collected
    for name in extern_names_moved:
        info = extern_opaque[name]
        all_remove_spans.append((info["start"], info["end"]))
    # Also remove extern opaque declarations for items that are being moved as real defs,
    # otherwise the local extern type shadows the imported struct.
    for name in local_names:
        if name in extern_opaque:
            info = extern_opaque[name]
            all_remove_spans.append((info["start"], info["end"]))
    new_source = source
    for start, end in sorted(all_remove_spans, key=lambda x: x[0], reverse=True):
        new_source = new_source[:start] + new_source[end:]

    # Add use statement for main item
    new_source = ensure_use_statement(new_source, item_name, use_prefix, dest_module_path)
    # Add use statements for deps still referenced in this file
    for name in deps_still_referenced:
        new_source = ensure_use_statement(new_source, name, use_prefix, dest_module_path)
    # Add use statements for moved extern types still referenced in source.
    # Use dest_module_path for now — main() may fix these if the extern type
    # ends up in a different module within the same crate.
    for name in extern_names_moved:
        if is_name_referenced(new_source, name):
            new_source = ensure_use_statement(new_source, name, use_prefix, dest_module_path)

    own_crate_pattern = re.compile(r'^\s*use\s+' + re.escape(import_crate_name(crate_name)) + r'::')
    dest_text = " ".join(extracted.values())
    use_stmts = [
        stmt
        for name, stmt in find_use_stmts_for_names(source, external_refs).items()
        if is_name_referenced(dest_text, name)
        and not own_crate_pattern.match(stmt)
    ]

    if deps_still_referenced:
        print(f"  use-imported in source (still referenced): {', '.join(sorted(deps_still_referenced))}")
    if use_stmts:
        print(f"  Carrying use statements: {len(use_stmts)}")

    return extracted, use_stmts, new_source


# -------------------------------
# Post-write: clean up redundant use statements in dest
# -------------------------------
def cleanup_redundant_use_stmts(dest_file):
    """
    Remove use statements in dest_file that import names which are now
    locally defined in the file (e.g. stale imports from a previous run).
    """
    source = dest_file.read_text(encoding="utf8")
    locally_defined = set(find_all_top_level_items(source).keys())
    if not locally_defined:
        return

    lines = source.splitlines()
    new_lines = []
    removed = []
    for line in lines:
        if line.strip().startswith("use "):
            imported = find_use_stmts_for_names(line, locally_defined)
            if imported:
                removed.append(line.strip())
                continue
        new_lines.append(line)

    if removed:
        dest_file.write_text("\n".join(new_lines) + "\n", encoding="utf8")
        for stmt in removed:
            print(f"  removed redundant: {stmt}")


# -------------------------------
# Post-write: resolve missing imports in dest
# -------------------------------
def find_crate_root(file_path):
    """Walk up from file_path to find the nearest directory containing Cargo.toml."""
    path = Path(file_path).resolve().parent
    while path != path.parent:
        if (path / "Cargo.toml").exists():
            return path
        path = path.parent
    return None


def resolve_missing_imports(dest_file, search_dirs, base_dir):
    """
    After items are written to dest_file, find type refs that have no local
    definition and no existing use statement, search the project for their
    definitions, and insert the required `use crate::...` statements.
    """
    source = dest_file.read_text(encoding="utf8")

    # Collect every type_identifier used in the file, excluding those inside
    # a scoped path (e.g. the `c_int` in `::core::ffi::c_int` is already qualified)
    tree = parser.parse(bytes(source, "utf8"))
    all_refs = set()

    def collect_refs(node):
        if node.type == "type_identifier":
            # Skip if this node is the `name` child of a scoped_type_identifier
            # (the path prefix already qualifies it fully)
            if node.parent and node.parent.type == "scoped_type_identifier":
                if node == node.parent.child_by_field_name("name"):
                    for child in node.children:
                        collect_refs(child)
                    return
            all_refs.add(source[node.start_byte:node.end_byte])
        for child in node.children:
            collect_refs(child)

    collect_refs(tree.root_node)

    locally_defined = set(find_all_top_level_items(source).keys())
    already_imported = set(find_use_stmts_for_names(source, all_refs).keys())
    missing = all_refs - locally_defined - already_imported

    if not missing:
        return

    # Restrict search to the same crate as dest_file so we don't pick up
    # definitions from sibling crates in the same workspace.
    crate_root = find_crate_root(dest_file)
    if crate_root:
        crate_src_dirs = [crate_root]
    else:
        crate_src_dirs = search_dirs

    # Search for each missing type's definition within the same crate
    new_use_stmts = []
    for name in sorted(missing):
        found = False
        for crate_dir in crate_src_dirs:
            for path in sorted(crate_dir.rglob("*.rs")):
                if path == dest_file:
                    continue
                try:
                    file_source = path.read_text(encoding="utf8")
                except OSError:
                    continue
                if find_item_nodes(file_source, name):
                    mod_path = module_path_from_file(path, base_dir)
                    use_stmt = f"use crate::{mod_path}::{name};"
                    if use_stmt not in source and use_stmt not in new_use_stmts:
                        new_use_stmts.append(use_stmt)
                        print(f"  auto import: {use_stmt}")
                    found = True
                    break
            if found:
                break

    if not new_use_stmts:
        return

    # Insert after the last existing `use` line (or at top if none)
    lines = source.splitlines()
    insert_idx = 0
    for i, line in enumerate(lines):
        if line.strip().startswith("use "):
            insert_idx = i + 1
    for stmt in reversed(new_use_stmts):
        lines.insert(insert_idx, stmt)
    dest_file.write_text("\n".join(lines) + "\n", encoding="utf8")


# -------------------------------
# Main logic
# -------------------------------
def main():
    if len(sys.argv) < 5:
        print("Usage:")
        print("  python deduplicate_struct.py <crate_name> <ItemName> <dest_file> <search_dir> [<search_dir>...]")
        sys.exit(1)

    crate_name = sys.argv[1]
    item_name = sys.argv[2]
    search_dirs = [Path(p).resolve() for p in sys.argv[4:]]

    dest_arg = Path(sys.argv[3])
    if dest_arg.is_absolute():
        dest_file = dest_arg.resolve()
    else:
        # Resolve relative dest path against the workspace root inferred from
        # the first search dir, not the cwd.
        workspace = find_crate_root(search_dirs[0])
        dest_file = (workspace / dest_arg).resolve() if workspace else dest_arg.resolve()

    base_dir = Path(os.path.commonpath(search_dirs))
    dest_module_path = module_path_from_file(dest_file, base_dir)
    dest_crate_root = find_crate_root(dest_file)

    # If the item already lives in dest_file, keep it there and only replace
    # definitions in other files with use statements (inline-dedup mode).
    dest_source = dest_file.read_text(encoding="utf8") if dest_file.exists() else ""
    if find_item_nodes(dest_source, item_name):
        print(f"'{item_name}' already in {dest_file} — replacing duplicates with use statements")
        for search_dir in search_dirs:
            for path in sorted(search_dir.rglob("*.rs")):
                if path.resolve() == dest_file:
                    continue
                source = path.read_text(encoding="utf8")
                nodes = find_item_nodes(source, item_name)
                if not nodes:
                    continue
                new_source = source
                for _, start, end in sorted(
                    [extract_item_with_attributes(source, n) for n in nodes],
                    key=lambda x: x[1], reverse=True,
                ):
                    new_source = new_source[:start] + new_source[end:]
                extern_opaque = find_foreign_opaque_types(new_source)
                if item_name in extern_opaque:
                    info = extern_opaque[item_name]
                    new_source = new_source[:info["start"]] + new_source[info["end"]:]
                source_crate_root = find_crate_root(path)
                use_prefix = "crate" if source_crate_root == dest_crate_root else import_crate_name(crate_name)
                new_source = ensure_use_statement(new_source, item_name, use_prefix, dest_module_path)
                path.write_text(new_source, encoding="utf8")
                cleanup_redundant_use_stmts(path)
                print(f"Updated: {path}")
        return

    # name -> list of text variants collected across all source files
    all_collected = {}
    all_use_stmts = []  # use statements to carry to destination

    for search_dir in search_dirs:
        for path in sorted(search_dir.rglob("*.rs")):
            if path == dest_file:
                continue  # dest is the canonical home — don't strip it as a source
            extracted, use_stmts, new_source = process_file(path, item_name, crate_name, dest_module_path, dest_crate_root)

            if extracted:
                for name, text in extracted.items():
                    all_collected.setdefault(name, []).append(text.strip())
                all_use_stmts.extend(use_stmts)

                with open(path, "w", encoding="utf8") as f:
                    f.write(new_source)

                print(f"Updated: {path}")

    if not all_collected:
        print("No definitions found.")
        return

    # If a name has both a real definition and extern opaque declarations,
    # discard the opaque ones (real definition takes precedence over forward decl).
    for name in list(all_collected.keys()):
        real = [t for t in all_collected[name] if not t.startswith('extern "C" {')]
        if real:
            all_collected[name] = real

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

    # Deduplicate collected use statements, skip ones already in dest
    unique_use_stmts = list(dict.fromkeys(s.strip() for s in all_use_stmts))

    existing_extern_types = set(find_foreign_opaque_types(existing).keys())

    # For deps (not the main item), check if they already exist in another module
    # of the same crate. If so, use `use crate::<module>::<name>;` instead of
    # writing a duplicate definition (avoids incompatible type errors).
    # Track redirected names so we can fix use statements in source files.
    dest_crate_root_resolved = find_crate_root(dest_file)
    extern_redirects = {}  # name -> correct module_path (within the crate)
    crate_files = {}
    if dest_crate_root_resolved:
        for path in sorted(dest_crate_root_resolved.rglob("*.rs")):
            if path.resolve() == dest_file:
                continue
            try:
                file_src = path.read_text(encoding="utf8")
            except OSError:
                continue
            crate_files[path] = file_src

        for name in list(canonical_defs.keys()):
            if name == item_name:
                continue  # never redirect the main item
            is_extern = canonical_defs[name].startswith('extern "C" {')
            found_in = None
            for path, file_src in crate_files.items():
                if is_extern:
                    if name in find_foreign_opaque_types(file_src):
                        found_in = path
                        break
                else:
                    if find_item_nodes(file_src, name):
                        found_in = path
                        break
            if found_in:
                mod_path = module_path_from_file(found_in, base_dir)
                use_stmt = f"use crate::{mod_path}::{name};"
                unique_use_stmts.append(use_stmt)
                del canonical_defs[name]
                write_order = [n for n in write_order if n != name]
                extern_redirects[name] = mod_path
                label = "extern type" if is_extern else "type"
                print(f"  {label} '{name}' already in crate → {use_stmt}")

    with open(dest_file, "a", encoding="utf8") as f:
        for stmt in unique_use_stmts:
            if stmt not in existing:
                f.write(stmt + "\n")
                existing += stmt + "\n"
                print(f"  use stmt: {stmt}")

        names_written = []
        for name in write_order:
            text = canonical_defs[name]
            if find_item_nodes(existing, name) or name in existing_extern_types:
                print(f"Already exists in destination: {name}")
            else:
                f.write("\n")
                f.write(text)
                existing += text  # update so subsequent names see it
                names_written.append(name)
                label = "moved" if name == item_name else "copied (dep)"
                print(f"  {label}: '{name}' → {dest_file}")

    cleanup_redundant_use_stmts(dest_file)
    resolve_missing_imports(dest_file, search_dirs, base_dir)

    # Clean up stale definitions in other crate files: if we just wrote a name to
    # the dest file, any other file in the same crate that has a duplicate definition
    # (from a previous iteration copying it as a dep) must have that definition removed
    # and replaced with `use crate::<dest_module>::<name>;`.
    if names_written and dest_crate_root_resolved:
        for path, file_src in crate_files.items():
            changed = False
            for name in names_written:
                is_extern = canonical_defs.get(name, "").startswith('extern "C" {')
                if is_extern:
                    extern_defs = find_foreign_opaque_types(file_src)
                    if name in extern_defs:
                        info = extern_defs[name]
                        file_src = file_src[:info["start"]] + file_src[info["end"]:]
                        use_stmt = f"use crate::{dest_module_path}::{name};"
                        if use_stmt not in file_src:
                            file_src = ensure_use_statement(file_src, name, "crate", dest_module_path)
                        changed = True
                        print(f"  cleaned stale extern type '{name}' in {path}")
                else:
                    nodes = find_item_nodes(file_src, name)
                    if nodes:
                        spans = [extract_item_with_attributes(file_src, n) for n in nodes]
                        for _, start, end in sorted(spans, key=lambda x: x[1], reverse=True):
                            file_src = file_src[:start] + file_src[end:]
                        use_stmt = f"use crate::{dest_module_path}::{name};"
                        if use_stmt not in file_src:
                            file_src = ensure_use_statement(file_src, name, "crate", dest_module_path)
                        changed = True
                        print(f"  cleaned stale def '{name}' in {path}")
            if changed:
                path.write_text(file_src, encoding="utf8")
                cleanup_redundant_use_stmts(path)

    # Fix source file use statements for extern types that got redirected to a
    # different module within the crate. E.g., source files may have gotten
    # `use ext_fts3_fts3::fts3_table::sqlite3;` but the type lives in fts3_cursor.
    if extern_redirects:
        use_crate = import_crate_name(crate_name)
        wrong_mod = dest_module_path
        for search_dir in search_dirs:
            for path in sorted(search_dir.rglob("*.rs")):
                if path.resolve() == dest_file:
                    continue
                try:
                    src = path.read_text(encoding="utf8")
                except OSError:
                    continue
                changed = False
                for name, correct_mod in extern_redirects.items():
                    wrong = f"use {use_crate}::{wrong_mod}::{name};"
                    right = f"use {use_crate}::{correct_mod}::{name};"
                    if wrong in src:
                        src = src.replace(wrong, right)
                        changed = True
                if changed:
                    path.write_text(src, encoding="utf8")

    # If any extern opaque types were written, ensure the crate's lib.rs has
    # #![feature(extern_types)] since those declarations require it.
    wrote_extern_types = any(
        t.startswith('extern "C" {')
        for texts in all_collected.values()
        for t in texts
    )
    if wrote_extern_types:
        crate_root = find_crate_root(dest_file)
        if crate_root:
            lib_rs = crate_root / "src" / "lib.rs"
            if lib_rs.exists():
                lib_src = lib_rs.read_text(encoding="utf8")
                feature = "#![feature(extern_types)]"
                if feature not in lib_src:
                    lib_rs.write_text(feature + "\n" + lib_src, encoding="utf8")
                    print(f"  added {feature} to {lib_rs}")


if __name__ == "__main__":
    main()
