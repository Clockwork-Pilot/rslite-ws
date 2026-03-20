import re
from pathlib import Path


def file_stem_to_crate_name(file_path):
    """Derive crate name from file path as-is: strip extension, replace / with -."""
    return Path(file_path).with_suffix("").as_posix().replace("/", "-").replace("src", "sql")


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
