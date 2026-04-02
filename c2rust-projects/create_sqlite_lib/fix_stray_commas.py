#!/usr/bin/env python3
"""
fix_stray_commas.py - Fix stray commas C2Rust generates in conditions and assignments.

Known C2Rust patterns:
  1. if (*db).u1.isInterrupted, != 0 {   (comma before operator, same line)
  2. if expr,                              (trailing comma, block on next line)
  3. (*pWal).minFrame = (*pInfo).nBackfill,
         .wrapping_add(1)                  (comma splits assignment + method call)
  4. (*pInfo).aReadMark[i], != mxReadMark (comma inside array-index condition)

Usage:
  python3 fix_stray_commas.py <src_dir>
"""
import os
import sys
import glob
import re


def fix_file(path: str) -> int:
    with open(path, 'r') as f:
        content = f.read()

    original = content

    # Fix 1: stray comma before comparison operator on same line
    # "expr, != 0"  ->  "expr != 0"
    content = re.sub(r',(\s*[!=<>]=?\s)', r'\1', content)

    # Fix 2: trailing comma on lines where the next line closes/opens a block
    # "if expr,\n    {"       ->  "if expr\n    {"
    # "return expr,\n    }"   ->  "return expr\n    }"
    # "return expr,\n    } else {" -> "return expr\n    } else {"
    lines = content.split('\n')
    result = []
    for i, line in enumerate(lines):
        stripped = line.rstrip()
        if stripped.endswith(',') and i + 1 < len(lines):
            next_stripped = lines[i + 1].strip()
            if next_stripped == '{' or next_stripped.startswith('}'):
                line = stripped[:-1] + '\n'
        result.append(line)
    content = '\n'.join(result)

    # Fix 3: comma that splits an assignment from a chained method call on next line
    # "= expr,\n    .method("  ->  "= expr\n    .method("
    content = re.sub(r',(\s*\n\s+\.\w+)', r'\1', content)

    if content != original:
        with open(path, 'w') as f:
            f.write(content)
        return 1
    return 0


def main(src_dir: str) -> None:
    rs_files = glob.glob(os.path.join(src_dir, '*.rs'))
    fixed = sum(fix_file(f) for f in rs_files)
    print(f"fix_stray_commas: fixed {fixed}/{len(rs_files)} files")


if __name__ == '__main__':
    src_dir = sys.argv[1] if len(sys.argv) > 1 else os.path.join(
        os.environ.get('OUTPUT_DIR', '/c2rust-projects/projects/minimal'), 'src')
    main(src_dir)
