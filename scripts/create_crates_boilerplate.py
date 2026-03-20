import json
import sys
from pathlib import Path

from crates_common import file_stem_to_crate_name


CARGO_TOML = """\
[package]
name = "{crate_name}"
version = "0.1.0"
edition = "2021"
"""


def main():
    if len(sys.argv) < 3:
        print("Usage: python create_crates_boilerplate.py <dupes.json> <crates_base_dir>")
        sys.exit(1)

    dupes_file = sys.argv[1]
    crates_base = Path(sys.argv[2]).resolve()

    with open(dupes_file, encoding="utf8") as f:
        data = json.load(f)

    # Collect unique .h c_decl_file values
    c_headers = sorted({
        info["c_decl_file"]
        for info in data["names"].values()
        if Path(info.get("c_decl_file", "")).suffix == ".h"
    })

    crate_names = []
    for c_header in c_headers:
        crate_name = file_stem_to_crate_name(c_header)
        crate_dir = crates_base / crate_name
        src_dir = crate_dir / "src"

        src_dir.mkdir(parents=True, exist_ok=True)

        cargo_toml = crate_dir / "Cargo.toml"
        if not cargo_toml.exists():
            cargo_toml.write_text(CARGO_TOML.format(crate_name=crate_name), encoding="utf8")
            print(f"created {crate_name}/Cargo.toml")

        lib_rs = src_dir / "lib.rs"
        if not lib_rs.exists():
            lib_rs.write_text("", encoding="utf8")
            print(f"created {crate_name}/src/lib.rs")

        crate_names.append(crate_name)

    print("\nworkspace.members")
    last = len(crate_names) - 1
    for i, crate_name in enumerate(crate_names):
        comma = "," if i < last else ""
        print(f'"crates/{crate_name}"{comma}')

    print("\nworkspace.dependencies")
    for crate_name in crate_names:
        print(f'{crate_name} = {{ path = "crates/{crate_name}" }}')


if __name__ == "__main__":
    main()
