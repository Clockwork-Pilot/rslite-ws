# Start
cd scripts

# Prerequisites
pip install tree_sitter
pip install tree-sitter-rust

# List types dups
python find_duplicates.py ~/git/sqlite-no-amalgam ../crust-sqlite/src/ > /tmp/dupes2.json

# Create crates boilerplate, and list copy-paste ready workspace artifacts
python create_crates_boilerplate.py /tmp/dupes2.json ../crust-sqlite/crates/

# Get list: <crate name> <type name> <move to new destination rs>
python show_one_of_many_dupes.py --deduplicate-type /tmp/dupes2.json

# Manually deduplicate one type item (debug with claude)
(cd ../crust-sqlite && git checkout src crates) && python deduplicate_struct.py ext_fts3_int Fts3Cursor crates/ext-fts3-int/src/lib.rs ../crust-sqlite/src/ && (cd ../crust-sqlite && cargo build)

# high level for  Claude debugging
python dedup_incrementally.py --build --suppress-build-warnings --explicit "sql_btree BtShared crates/sql-btree/src/lib.rs" /tmp/dupes2.json progress.txt ../crust-sqlite/

# Non Stop: Deduplicate in a loop one by one, commit successfull attempts
(cd ../crust-sqlite && git checkout -- src crates) && while python dedup_incrementally.py --build --suppress-build-warnings /tmp/dupes2.json progress.txt ../crust-sqlite/ && (cd ../crust-sqlite && git add src crates && git commit -m "$(tail -n 1 ../scripts/progress.txt)" || echo "Do not commit on error"); do :; done

# DRY RUN
(cd ../crust-sqlite && git checkout -- src crates) && while python dedup_incrementally.py --skip-errors /tmp/dupes2.json progress.txt ../crust-sqlite/ && (cd ../crust-sqlite && cargo build && git add -- src crates || echo "FAILED: $(tail -n 1 ../scripts/progress.txt)" ); do :; done


# To fix errors related to private declarations:
(deduplicate_struct.py):
      657 -            text = canonical_defs[name]
      657 +            text = ensure_pub(canonical_defs[name])

