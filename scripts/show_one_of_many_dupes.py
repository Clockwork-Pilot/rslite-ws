import json
import sys
from pathlib import Path

from crates_common import file_stem_to_crate_name, import_crate_name


def build_by_file(data):
    by_file = {}
    for name, info in data["names"].items():
        if len(info["occurs"]) <= 1:
            continue
        for occur in info["occurs"]:
            if occur["count"] == 1:
                by_file.setdefault(occur["file"], []).append(name)
    return by_file


def main():
    args = sys.argv[1:]

    deduplicate = "--deduplicate-type" in args
    input_file = next((a for a in args if not a.startswith("--")), None)

    if input_file:
        with open(input_file, encoding="utf8") as f:
            data = json.load(f)
    else:
        data = json.load(sys.stdin)

    if deduplicate:
        for name, info in data["names"].items():
            if len(info["occurs"]) <= 1:
                continue
            c_decl = info.get("c_decl_file", "")
            if Path(c_decl).suffix == ".h":
                crate_name = file_stem_to_crate_name(c_decl)
                use_name = import_crate_name(crate_name)
                dest = f"crates/{crate_name}/src/lib.rs"
                print(f"{use_name} {name} {dest}")
            elif c_decl:
                c_stem = Path(c_decl).with_suffix("").as_posix()
                rs_file = next(
                    (o["file"] for o in info["occurs"]
                     if Path(o["file"]).with_suffix("").as_posix() == c_stem),
                    None,
                )
                if rs_file:
                    crate_name = file_stem_to_crate_name(rs_file)
                    use_name = import_crate_name(crate_name)
                    print(f"{use_name} {name} {rs_file}")
    else:
        by_file = build_by_file(data)
        for file, names in sorted(by_file.items()):
            print(file)
            for name in names:
                print(f"  {name}")


if __name__ == "__main__":
    main()
