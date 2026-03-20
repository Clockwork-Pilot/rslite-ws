import json
import sys

from crates_common import file_stem_to_crate_name, rs_file_name_from_type_name


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

    mode = None
    mode_arg = None
    input_file = None

    for arg in args:
        if arg == "--list-files":
            mode = "list-files"
        elif arg.startswith("--list-types-for-file="):
            mode = "list-types"
            mode_arg = arg.split("=", 1)[1]
        elif arg == "--deduplicate-type":
            mode = "deduplicate-type"
        else:
            input_file = arg

    if input_file:
        with open(input_file, encoding="utf8") as f:
            data = json.load(f)
    else:
        data = json.load(sys.stdin)

    by_file = build_by_file(data)

    if mode == "list-files":
        for file in sorted(by_file.keys()):
            print(file)
    elif mode == "list-types":
        names = by_file.get(mode_arg, [])
        for name in names:
            print(name)
    elif mode == "deduplicate-type":
        for file_path in sorted(by_file.keys()):
            crate_name = file_stem_to_crate_name(file_path)
            for name in by_file[file_path]:
                file_name = rs_file_name_from_type_name(name)
                rs_file = f"crates/{crate_name}/src/{file_name}.rs"
                print(f"{crate_name} {name} {rs_file}")
    else:
        for file, names in sorted(by_file.items()):
            print(file)
            for name in names:
                print(f"  {name}")


if __name__ == "__main__":
    main()
