#!/usr/bin/env python3
"""
fix_atomics.py - Remove/replace C2Rust-generated atomic intrinsic calls.

C2Rust sometimes generates calls to unstable intrinsics:
  ::core::intrinsics::atomic_fence_seqcst()
  ::core::intrinsics::atomic_load_relaxed(&raw mut x)
  ::core::intrinsics::atomic_store_relaxed(&raw mut x, val)

These are removed or simplified for nightly compatibility.

Usage:
  python3 fix_atomics.py <src_dir>
"""
import os
import sys
import glob
import re


def extract_args(content: str, call_start: int, fn_name: str) -> tuple[str, int]:
    """Return (inner_text, end_index) for the call starting at call_start."""
    open_paren = call_start + len(fn_name) + 1  # +1 for '('
    i = open_paren
    depth = 1
    while i < len(content) and depth > 0:
        if content[i] == '(':
            depth += 1
        elif content[i] == ')':
            depth -= 1
        i += 1
    return content[open_paren:i - 1], i


def fix_file(path: str) -> int:
    with open(path, 'r') as f:
        content = f.read()

    original = content

    # Remove atomic_fence_seqcst();
    content = content.replace('::core::intrinsics::atomic_fence_seqcst();', '')

    # Replace atomic_load_relaxed(&raw mut x) -> x
    fence = '::core::intrinsics::atomic_load_relaxed('
    while fence in content:
        start = content.find(fence)
        arg, end = extract_args(content, start, '::core::intrinsics::atomic_load_relaxed')
        arg = arg.replace('&raw mut ', '', 1).strip()
        content = content[:start] + arg + content[end:]

    # Remove atomic_store_relaxed(&raw mut x, val) entirely
    fence2 = '::core::intrinsics::atomic_store_relaxed('
    while fence2 in content:
        start = content.find(fence2)
        _, end = extract_args(content, start, '::core::intrinsics::atomic_store_relaxed')
        content = content[:start] + content[end:]

    # Clean up trailing comma artifacts from removal
    content = content.replace(',;', ';')
    content = re.sub(r',\s+as\s+', ' as ', content)
    content = content.replace(',}', '}')
    content = content.replace(',)', ')')
    content = re.sub(r'=\s*\(([^)]+)\),\s*;', r'= \1;', content)
    content = re.sub(r'(\w+)\s*,\s*,', r'\1,', content)

    if content != original:
        with open(path, 'w') as f:
            f.write(content)
        return 1
    return 0


def main(src_dir: str) -> None:
    rs_files = glob.glob(os.path.join(src_dir, '*.rs'))
    rs_files = [f for f in rs_files if not f.endswith('lib.rs')]
    fixed = sum(fix_file(f) for f in rs_files)
    print(f"fix_atomics: fixed {fixed}/{len(rs_files)} files")


if __name__ == '__main__':
    src_dir = sys.argv[1] if len(sys.argv) > 1 else os.path.join(
        os.environ.get('OUTPUT_DIR', '/c2rust-projects/projects/minimal'), 'src')
    main(src_dir)
