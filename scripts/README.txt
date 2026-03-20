# Example of moving particular struct
See commands.sh

# Prerequisites
pip install tree_sitter
pip install tree-sitter-rust

# List types dups
python find_duplicates.py ../crust-sqlite/src/ > /tmp/dupes.json

# Create crates boilerplate, and list copy-paste ready workspace artifacts
python create_crates_boilerplate.py /tmp/dupes.json ../crust-sqlite/crates/

# Manually move one
(cd ../crust-sqlite && git checkout src crates) && python deduplicate_struct.py ext-fts3-fts3 Fts3Cursor crates/ext-fts3-fts3/src/fts3_cursor.rs src/ && (cd ../crust-sqlite && cargo build)

# Move one type at a time
while python dedup_incrementally.py /tmp/dupes.json progress.txt ../crust-sqlite/ && (cd ../crust-sqlite && cargo build && tail -n 1 progress.txt); do :; done

(cd ../crust-sqlite && git checkout src crates) && while python dedup_incrementally.py /tmp/dupes.json progress.txt ../crust-sqlite/ && (cd ../crust-sqlite && cargo build && tail -n 1 scripts/progress.txt); do :; done

# Final all in one command
(cd .. && python scripts/dedup_incrementally.py /tmp/dupes.json progress.txt ../crust-sqlite)
while python dedup_incrementally.py /tmp/dupes.json progress.txt ../crust-sqlite/; do :; done


