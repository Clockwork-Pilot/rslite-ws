# Example of moving particular struct
See commands.sh

# Prerequisites
pip install tree_sitter
pip install tree-sitter-rust

# List types dups
python find_duplicates.py ../crust-sqlite/src/ > /tmp/dupes.json

# Get list: <crate name> <type name> <move to new destination rs>
python show_one_of_many_dupes.py --deduplicate-type /tmp/dupes.json

# Create crates boilerplate, and list copy-paste ready workspace artifacts
python create_crates_boilerplate.py /tmp/dupes.json ../crust-sqlite/crates/

# Manually deduplicate one type item (debug with claude)
(cd ../crust-sqlite && git checkout src crates) && python deduplicate_struct.py ext-fts3-fts3 Fts3Cursor crates/ext-fts3-fts3/src/fts3_cursor.rs src/ && (cd ../crust-sqlite && cargo build)

# All-in-one: Deduplicate in a loop one by one, commit successfull attempts

## Stop on first error:
(cd ../crust-sqlite && git checkout -- src crates) && while python dedup_incrementally.py /tmp/dupes.json progress.txt ../crust-sqlite/ && (cd ../crust-sqlite && cargo build && git add -- src crates && git commit -m "$(tail -n 1 ../scripts/progress.txt)"); do :; done

## Continue past errors NON STOP:
(cd ../crust-sqlite && git checkout -- src crates) && while python dedup_incrementally.py --skip-errors /tmp/dupes.json progress.txt ../crust-sqlite/ && (cd ../crust-sqlite && cargo build && git add -- src crates && git commit -m "$(tail -n 1 ../scripts/progress.txt)" || git checkout -- src crates); do :; done

