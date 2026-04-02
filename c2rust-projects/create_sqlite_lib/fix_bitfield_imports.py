#!/usr/bin/env python3
"""
fix_bitfield_imports.py - Add missing BitfieldStruct import to modules that use it.

C2Rust transpiles code that uses c2rust_bitfields derive macros. In Rust 2021 edition,
proc-macro derives (like BitfieldStruct) must be explicitly imported in each module.

The lib.rs `#[macro_use] extern crate c2rust_bitfields;` does NOT bring proc-macro
derives into scope for submodules. Each module that uses `#[derive(BitfieldStruct)]`
needs `use c2rust_bitfields::BitfieldStruct;`.

This script also replaces `use ::c2rust_bitfields;` (which only imports the crate
but no items) with the correct import.

Usage:
  python3 fix_bitfield_imports.py <src_dir>
"""
import os
import sys
import glob


def fix_file(path: str) -> int:
    with open(path, 'r') as f:
        content = f.read()

    # Only process files that actually use BitfieldStruct derive
    if 'BitfieldStruct' not in content:
        return 0

    original = content

    # Replace `use ::c2rust_bitfields;` with the correct item import
    content = content.replace(
        'use ::c2rust_bitfields;\n',
        'use c2rust_bitfields::BitfieldStruct;\n'
    )

    # If the file uses BitfieldStruct but doesn't have the import yet, add it
    if ('BitfieldStruct' in content
            and 'use c2rust_bitfields::BitfieldStruct;' not in content):
        # Insert after the first line (or at the very top)
        lines = content.split('\n')
        lines.insert(0, 'use c2rust_bitfields::BitfieldStruct;')
        content = '\n'.join(lines)

    if content != original:
        with open(path, 'w') as f:
            f.write(content)
        return 1
    return 0


def main(src_dir: str) -> None:
    rs_files = glob.glob(os.path.join(src_dir, '*.rs'))
    # Don't touch lib.rs - it has the extern crate declaration
    rs_files = [f for f in rs_files if not f.endswith('lib.rs')]
    fixed = sum(fix_file(f) for f in rs_files)
    print(f"fix_bitfield_imports: fixed {fixed}/{len(rs_files)} files")


if __name__ == '__main__':
    src_dir = sys.argv[1] if len(sys.argv) > 1 else os.path.join(
        os.environ.get('OUTPUT_DIR', '/c2rust-projects/projects/minimal'), 'src')
    main(src_dir)
