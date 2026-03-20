import subprocess
import sys
from pathlib import Path

SCRIPTS_DIR = Path(__file__).parent


def load_accumulator(acc_file):
    if not Path(acc_file).exists():
        return set()
    lines = Path(acc_file).read_text(encoding="utf8").splitlines()
    # Strip trailing FAILED marker to get the base line for comparison
    return {line.removesuffix(" FAILED") for line in lines if line.strip()}


def get_dedup_lines(dedups_file):
    result = subprocess.run(
        [sys.executable, SCRIPTS_DIR / "show_one_of_many_dupes.py",
         "--deduplicate-type", dedups_file],
        capture_output=True, text=True
    )
    return [l for l in result.stdout.splitlines() if l.strip()]


def main():
    args = sys.argv[1:]
    skip_errors = "--skip-errors" in args
    if skip_errors:
        args.remove("--skip-errors")

    if len(args) < 3:
        print("Usage: python dedup_incrementally.py [--skip-errors] <dedups.json> <accumulator.txt> <crust_sqlite_dir>")
        sys.exit(1)

    dedups_file = args[0]
    acc_file = args[1]
    base_dir = Path(args[2]).resolve()

    done = load_accumulator(acc_file)
    all_lines = get_dedup_lines(dedups_file)

    pending = next((l for l in all_lines if l not in done), None)
    if pending is None:
        print("Nothing left to deduplicate.", file=sys.stderr)
        sys.exit(1)

    parts = pending.split()
    crate_name, type_name, dest_file = parts[0], parts[1], parts[2]

    print(f"Processing: {pending}")

    result = subprocess.run(
        [sys.executable, SCRIPTS_DIR / "deduplicate_struct.py",
         crate_name, type_name, dest_file, str(base_dir)],
    )

    with open(acc_file, "a", encoding="utf8") as f:
        if result.returncode != 0:
            f.write(pending + " FAILED\n")
            print(f"FAILED: {pending}")
            sys.exit(0 if skip_errors else 2)
        else:
            f.write(pending + "\n")
            print(f"OK: {pending}")
            sys.exit(0)


if __name__ == "__main__":
    main()
