#!/bin/bash

while (cd ../crust-sqlite && git checkout -- src crates); do
  python dedup_incrementally.py --build --suppress-build-warnings /tmp/dupes2.json progress.txt ../crust-sqlite/
  rc=$?
  [ $rc -eq 1 ] && break
  [ $rc -eq 0 ] && (cd ../crust-sqlite && git add src crates && git commit -m "$(tail -n 1 ../scripts/progress.txt)")
done
