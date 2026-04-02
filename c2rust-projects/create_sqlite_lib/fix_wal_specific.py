#!/usr/bin/env python3
"""
fix_wal_specific.py - Fix wal.rs-specific C2Rust artifacts.

C2Rust generates:
  1. ((*pInfo).aReadMark as *mut u32_0).offset(i as isize)
     Should be: (*pInfo).aReadMark[i as usize]
     (This field IS a fixed-size array; C2Rust mistakenly converts it to pointer arithmetic)

  2. (*pWal).minFrame = (*pInfo).nBackfill,
         .wrapping_add(1);
     Should be: (*pWal).minFrame = (*pInfo).nBackfill
         .wrapping_add(1);
     (Stray comma splits assignment from chained method call)

Usage:
  python3 fix_wal_specific.py <src_dir>
"""
import os
import sys
import re


def fix_file(path: str) -> int:
    with open(path, 'r') as f:
        content = f.read()

    original = content

    # Fix 1: aReadMark pointer arithmetic -> array indexing
    # ((*pInfo).aReadMark as *mut u32_0).offset(VAR as isize)
    # -> (*pInfo).aReadMark[VAR as usize]
    # aReadMark is a fixed-size array [u32; 5]; C2Rust wrongly casts it to a pointer.
    content = re.sub(
        r'\(\(\*pInfo\)\.aReadMark\s+as\s+\*mut\s+u32_0\)\.offset\((\w+)\s+as\s+isize\)',
        r'(*pInfo).aReadMark[\1 as usize]',
        content
    )

    # Fix 2: stray comma before chained .wrapping_add (multiline)
    # nBackfill,\n    .wrapping_add  ->  nBackfill\n    .wrapping_add
    content = re.sub(
        r'(\.nBackfill),(\s*\n\s*\.wrapping_add)',
        r'\1\2',
        content
    )

    if content != original:
        with open(path, 'w') as f:
            f.write(content)
        return 1
    return 0


def main(src_dir: str) -> None:
    wal_path = os.path.join(src_dir, 'wal.rs')
    if not os.path.exists(wal_path):
        print("fix_wal_specific: wal.rs not found, skipping")
        return
    fixed = fix_file(wal_path)
    print(f"fix_wal_specific: {'fixed' if fixed else 'no changes in'} wal.rs")


if __name__ == '__main__':
    src_dir = sys.argv[1] if len(sys.argv) > 1 else os.path.join(
        os.environ.get('OUTPUT_DIR', '/c2rust-projects/projects/minimal'), 'src')
    main(src_dir)
