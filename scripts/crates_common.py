import json
import re
from pathlib import Path

try:
    import tomllib  # Python 3.11+
except ImportError:
    try:
        import tomli as tomllib  # Fallback for older Python
    except ImportError:
        tomllib = None  # Will use regex fallback


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


def add_workspace_dependency(cargo_toml_path, crate_name_dash):
    """Add a workspace crate to [dependencies] in Cargo.toml.

    Uses tomllib to verify TOML structure, regex for safe insertion.

    Args:
        cargo_toml_path: Path to the crate's Cargo.toml
        crate_name_dash: Crate name in dash-case (e.g., 'sql-where-int')

    Returns:
        True if added, False if already exists
    """
    cargo_path = Path(cargo_toml_path)
    if not cargo_path.exists():
        return False

    content = cargo_path.read_text(encoding="utf8")

    # Use tomllib to verify it's valid TOML and check for existing dependency
    if tomllib:
        try:
            import io
            data = tomllib.loads(content)
            if "dependencies" in data and crate_name_dash in data["dependencies"]:
                return False  # Already exists
        except Exception:
            pass  # Continue with regex fallback

    # Check if dependency already exists via regex
    if re.search(rf'^\s*{re.escape(crate_name_dash)}\s*=', content, re.MULTILINE):
        return False

    # Ensure [dependencies] section exists
    if '[dependencies]' not in content:
        content += '\n[dependencies]\n'

    # Add dependency with relative path
    dep_line = f'{crate_name_dash} = {{ workspace = true }}\n'

    # Insert right after [dependencies] line
    content = re.sub(
        r'(\[dependencies\]\n)',
        rf'\1{dep_line}',
        content,
        count=1
    )

    cargo_path.write_text(content, encoding="utf8")
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
        dict of name -> {"count": N, "files": [...], "differences": [...]}
    """
    report = {}

    for name, definitions in definitions_dict.items():
        unique_defs = list(dict.fromkeys(defs for _file, defs in definitions))

        if len(unique_defs) <= 1:
            continue  # No conflict

        # Analyze what's different
        differences = []

        # Line-by-line diff
        lines_by_def = [d.split('\n') for d in unique_defs]
        max_lines = max(len(lines) for lines in lines_by_def)

        for i in range(max_lines):
            line_set = set()
            for lines in lines_by_def:
                if i < len(lines):
                    line_set.add(lines[i])

            if len(line_set) > 1:
                differences.append({
                    "line": i + 1,
                    "variants": list(line_set)
                })

        files = [f for f, _defs in definitions]

        report[name] = {
            "count": len(unique_defs),
            "files": files,
            "differences": differences,
            "is_trivial": len(differences) <= 2  # Minor field name changes are trivial
        }

    return report


def format_conflict_report(report):
    """Format conflict analysis as readable text."""
    if not report:
        return "No conflicts detected."

    lines = []
    lines.append(f"\nCONFLICTS DETECTED ({len(report)} types):\n")

    for name, info in sorted(report.items()):
        is_trivial = info["is_trivial"]
        tag = "[TRIVIAL]" if is_trivial else "[SIGNIFICANT]"

        lines.append(f"{tag} {name}")
        lines.append(f"  Files: {', '.join(info['files'])}")
        lines.append(f"  Variants: {info['count']}")

        if info["differences"]:
            lines.append(f"  Differences:")
            for diff in info["differences"][:5]:  # Show first 5 differences
                lines.append(f"    Line {diff['line']}:")
                for variant in diff["variants"]:
                    lines.append(f"      - {variant[:70]}")
            if len(info["differences"]) > 5:
                lines.append(f"    ... and {len(info['differences']) - 5} more")

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
