import difflib
import json
import re
from pathlib import Path

import tomlkit


def _find_workspace_root(start_dir):
    """Walk up from start_dir to find the workspace root directory."""
    path = Path(start_dir).resolve()
    while path != path.parent:
        candidate = path / "Cargo.toml"
        if candidate.exists():
            try:
                doc = tomlkit.parse(candidate.read_text(encoding="utf8"))
                if "workspace" in doc:
                    return path
            except OSError:
                pass
        path = path.parent
    return None


def file_stem_to_crate_name(file_path):
    """Derive dash-case crate name from a file path (strips extension, camelCase → dash-case)."""
    parts = Path(file_path).with_suffix("").parts
    dash_parts = []
    for part in parts:
        s = re.sub(r"(?<=[A-Z])(?=[A-Z][a-z])|(?<=[a-z\d])(?=[A-Z])", "-", part)
        dash_parts.append(s.lower().replace("_", "-"))
    joined = "-".join(dash_parts).replace("src", "sql")
    return re.sub(r"\b(\w+)-\1\b", r"\1", joined)


def import_crate_name(crate_name):
    """Convert crate name to Rust import name (hyphens → underscores)."""
    return crate_name.replace("-", "_")


def dash_crate_name(crate_name):
    """Convert crate name to Cargo.toml dash-case (underscores → hyphens)."""
    return crate_name.replace("_", "-")


def rs_file_name_from_type_name(name):
    """Return snake_case filename for a type name.
    If already snake_case, returns as-is. CamelCase is converted to snake_case.
    """
    # Already snake_case if fully lowercase (digits and underscores are fine)
    if name == name.lower():
        return name
    # CamelCase → snake_case
    s = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", name)
    s = re.sub(r"([a-z\d])([A-Z])", r"\1_\2", s)
    return s.lower()


def dep_crate_depends_on(new_dep_dash, dest_crate_dash, cargo_toml_path):
    """Return True if the crate named new_dep_dash already (directly) depends on dest_crate_dash.

    Searches all Cargo.toml files under the workspace root.
    """
    new_dep_dash = dash_crate_name(new_dep_dash)
    dest_crate_dash = dash_crate_name(dest_crate_dash)
    if new_dep_dash == dest_crate_dash:
        return True  # self-reference is always a cycle

    ws_root = _find_workspace_root(Path(cargo_toml_path).resolve().parent)
    if ws_root is None:
        return False

    dest_underscore = dest_crate_dash.replace("-", "_")
    for cargo_path in sorted(ws_root.rglob("Cargo.toml")):
        try:
            doc = tomlkit.parse(cargo_path.read_text(encoding="utf8"))
        except OSError:
            continue
        pkg = doc.get("package", {})
        if dash_crate_name(pkg.get("name", "")) != new_dep_dash:
            continue
        for section in ("dependencies", "dev-dependencies", "build-dependencies"):
            deps = doc.get(section, {})
            if dest_crate_dash in deps or dest_underscore in deps:
                return True
        return False  # found the crate's Cargo.toml, it doesn't depend on dest
    return False


def add_workspace_dependency(cargo_toml_path, crate_name_dash):
    """Add a workspace crate to [dependencies] in Cargo.toml.

    Args:
        cargo_toml_path: Path to the crate's Cargo.toml
        crate_name_dash: Crate name (dash-case or underscore-case)

    Returns:
        True if added, False if already exists or would create a cycle
    """
    crate_name_dash = dash_crate_name(crate_name_dash)
    cargo_path = Path(cargo_toml_path)
    if not cargo_path.exists():
        return False

    doc = tomlkit.parse(cargo_path.read_text(encoding="utf8"))
    deps = doc.get("dependencies", {})
    if crate_name_dash in deps:
        return False

    # Guard: only add {workspace = true} if crate is in [workspace.dependencies]
    ws_root = _find_workspace_root(cargo_path.parent)
    if ws_root:
        try:
            ws_doc = tomlkit.parse((ws_root / "Cargo.toml").read_text(encoding="utf8"))
            ws_deps = ws_doc.get("workspace", {}).get("dependencies", {})
            if crate_name_dash not in ws_deps and crate_name_dash.replace("-", "_") not in ws_deps:
                print(f"  SKIP dep '{crate_name_dash}': not in workspace.dependencies")
                return False
        except OSError:
            pass

    # Cycle guard: skip if the new dep already depends on this crate
    dest_crate_dash = dash_crate_name(doc.get("package", {}).get("name", ""))
    if dest_crate_dash and dep_crate_depends_on(crate_name_dash, dest_crate_dash, cargo_toml_path):
        print(f"  SKIP dep '{crate_name_dash}': would create a cycle with '{dest_crate_dash}'")
        return False

    mutable_deps = doc.setdefault("dependencies", tomlkit.table())
    entry = tomlkit.inline_table()
    entry.append("workspace", True)
    mutable_deps[crate_name_dash] = entry
    cargo_path.write_text(tomlkit.dumps(doc), encoding="utf8")
    return True


def load_crate_requirements():
    """Load crate requirements from crate-requirements.json."""
    req_file = Path(__file__).parent / "crate-requirements.json"
    if not req_file.exists():
        return []

    with open(req_file, encoding="utf8") as f:
        data = json.load(f)
    return data.get("requirements", [])


def analyze_conflicting_definitions(definitions_dict):
    """Analyze differences between conflicting definitions.

    Args:
        definitions_dict: dict of name -> list of (source_file, definition_text)

    Returns:
        dict of name -> {"variants": [(loc, text), ...], "diff": unified_diff_str}
    """
    report = {}

    for name, definitions in definitions_dict.items():
        # Deduplicate while preserving (loc, text) pairs
        seen = {}
        for loc, text in definitions:
            if text not in seen:
                seen[text] = loc
        variants = [(loc, text) for text, loc in seen.items()]

        if len(variants) <= 1:
            continue

        # Build unified diff between first two variants
        a_loc, a_text = variants[0]
        b_loc, b_text = variants[1]
        diff_lines = list(difflib.unified_diff(
            a_text.splitlines(keepends=True),
            b_text.splitlines(keepends=True),
            fromfile=a_loc,
            tofile=b_loc,
        ))
        diff_str = "".join(diff_lines)

        report[name] = {
            "variants": variants,
            "diff": diff_str,
        }

    return report


def format_conflict_report(report):
    """Format conflict analysis as readable text."""
    if not report:
        return "No conflicts detected."

    lines = []
    lines.append(f"\nCONFLICTS DETECTED ({len(report)} types):\n")

    for name, info in sorted(report.items()):
        variants = info["variants"]
        lines.append(f"{'=' * 60}")
        lines.append(f"CONFLICT: {name}  ({len(variants)} versions)")
        lines.append("")

        for i, (loc, text) in enumerate(variants, 1):
            lines.append(f"  --- version {i}: {loc}")
            for src_line in text.splitlines():
                lines.append(f"    {src_line}")
            lines.append("")

        if info["diff"]:
            lines.append("  diff (version 1 → version 2):")
            for diff_line in info["diff"].splitlines():
                lines.append(f"    {diff_line}")
            lines.append("")

    return "\n".join(lines)


def detect_required_features(content):
    """Detect which crate requirements are needed based on content patterns.

    Returns dict with keys:
    - "features": list of Rust features to enable
    - "uses": list of use statements to add
    - "dependencies": dict of crate_name -> version
    """
    requirements = load_crate_requirements()
    result = {"features": [], "uses": [], "dependencies": {}}

    for req in requirements:
        patterns = req.get("patterns", [])
        if any(pattern in content for pattern in patterns):
            result["features"].extend(req.get("features", []))
            result["uses"].extend(req.get("uses", []))
            result["dependencies"].update(req.get("dependencies", {}))

    # Remove duplicates while preserving order
    result["features"] = list(dict.fromkeys(result["features"]))
    result["uses"] = list(dict.fromkeys(result["uses"]))

    return result
