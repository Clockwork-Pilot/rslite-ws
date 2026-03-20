import json
import sys
from pathlib import Path

from show_one_of_many_dupes import build_by_file
from crates_common import file_stem_to_crate_name, rs_file_name_from_type_name


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

    by_file = build_by_file(data)

    crate_names = []
    for file_path in sorted(by_file.keys()):
        crate_name = file_stem_to_crate_name(file_path)
        crate_dir = crates_base / crate_name
        src_dir = crate_dir / "src"
        type_names = by_file[file_path]

        if not crate_dir.exists():
            src_dir.mkdir(parents=True)
            print(f"created crate: {crate_name}")

        cargo_toml = crate_dir / "Cargo.toml"
        if not cargo_toml.exists():
            cargo_toml.write_text(CARGO_TOML.format(crate_name=crate_name), encoding="utf8")
            print(f"  created Cargo.toml")

        if not src_dir.exists():
            src_dir.mkdir(parents=True)

        for name in type_names:
            file_name = rs_file_name_from_type_name(name)
            rs_file = src_dir / f"{file_name}.rs"
            if not rs_file.exists():
                rs_file.write_text("", encoding="utf8")
                print(f"  created src/{file_name}.rs")

        lib_rs = src_dir / "lib.rs"
        existing_lib = lib_rs.read_text(encoding="utf8") if lib_rs.exists() else ""
        new_mods = [
            f"pub mod {rs_file_name_from_type_name(name)};"
            for name in type_names
            if f"pub mod {rs_file_name_from_type_name(name)};" not in existing_lib
        ]
        if new_mods:
            with open(lib_rs, "a", encoding="utf8") as f:
                f.write("\n".join(new_mods) + "\n")
            for mod in new_mods:
                print(f"  added to lib.rs: {mod}")

        crate_names.append(crate_name)

    print("workspace.members")
    last = len(crate_names) - 1
    for i, crate_name in enumerate(crate_names):
        comma = "," if i < last else ""
        print(f'"crates/{crate_name}"{comma}')

    print("\nworkspace.dependencies")
    for crate_name in crate_names:
        print(f'{crate_name} = {{ path = "crates/{crate_name}" }}')


if __name__ == "__main__":
    main()
