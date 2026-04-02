#!/usr/bin/env python3
"""
fix_match_arms.py - Fix match arm return statements missing braces.

C2Rust generates:
  _ => return expr;

Rust requires a block body when there are multiple arm types:
  _ => { return expr; }

Usage:
  python3 fix_match_arms.py <src_dir>
"""
import os
import sys
import glob
import re


def fix_file(path: str) -> int:
    with open(path, 'r') as f:
        lines = f.readlines()

    result = []
    changed = False
    for line in lines:
        # Match: "    PATTERN => return EXPR;"  (not already in braces)
        # Pattern must be simple: single word, underscore, or word|word
        m = re.match(r'^(\s+)(\w+(?:\s*\|\s*\w+)*)\s*=>\s+(return\s+[^;{]+);$', line.rstrip('\n'))
        if m:
            indent, pattern, return_stmt = m.group(1), m.group(2), m.group(3)
            result.append(f'{indent}{pattern} => {{ {return_stmt}; }}\n')
            changed = True
        else:
            result.append(line)

    if changed:
        with open(path, 'w') as f:
            f.writelines(result)
        return 1
    return 0


def main(src_dir: str) -> None:
    rs_files = glob.glob(os.path.join(src_dir, '*.rs'))
    fixed = sum(fix_file(f) for f in rs_files)
    print(f"fix_match_arms: fixed {fixed}/{len(rs_files)} files")


if __name__ == '__main__':
    src_dir = sys.argv[1] if len(sys.argv) > 1 else os.path.join(
        os.environ.get('OUTPUT_DIR', '/c2rust-projects/projects/minimal'), 'src')
    main(src_dir)
