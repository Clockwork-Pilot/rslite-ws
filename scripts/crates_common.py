import json
import re
from pathlib import Path


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


def load_crate_requirements():
    """Load crate requirements from crate-requirements.json."""
    req_file = Path(__file__).parent / "crate-requirements.json"
    if not req_file.exists():
        return []

    with open(req_file, encoding="utf8") as f:
        data = json.load(f)
    return data.get("requirements", [])


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
