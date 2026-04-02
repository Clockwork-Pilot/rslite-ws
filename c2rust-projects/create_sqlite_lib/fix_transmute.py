#!/usr/bin/env python3
"""
fix_transmute.py - Fix transmute calls missing explicit destination type.

C2Rust generates (possibly multiline, with nested angle brackets):
  LHS = ::core::mem::transmute(EXPR) as DEST_TYPE;

Rust needs an explicit destination type parameter:
  LHS = ::core::mem::transmute::<_, DEST_TYPE>(EXPR);

We add `_` for the source and let the compiler infer it from EXPR.

Usage:
  python3 fix_transmute.py <src_dir>
"""
import os
import sys
import glob


def extract_balanced_parens(content: str, start: int) -> int:
    """Return index just after matching ')' of opening '(' before `start`.
    `start` points to the char after the opening '('."""
    depth = 1
    i = start
    while i < len(content) and depth > 0:
        if content[i] == '(':
            depth += 1
        elif content[i] == ')':
            depth -= 1
        i += 1
    return i  # points to char after closing ')'


def skip_whitespace(content: str, pos: int) -> int:
    while pos < len(content) and content[pos] in ' \t\n\r':
        pos += 1
    return pos


def extract_dest_type(content: str, pos: int):
    """Parse `as DEST_TYPE;` starting at `pos`, allowing any character
    except `;` as the type body (balanced angle brackets / parens).
    Returns (dest_type_str, end_pos) or (None, pos) if no match."""
    p = skip_whitespace(content, pos)
    if not content[p:p+3] == 'as ':
        return None, pos

    p += 3
    p = skip_whitespace(content, p)

    # Read until unambiguous semicolon (not inside <>, (), [])
    depth_angle = 0
    depth_paren = 0
    depth_bracket = 0
    start = p
    while p < len(content):
        c = content[p]
        if c == '<':
            depth_angle += 1
        elif c == '>':
            if depth_angle > 0:
                depth_angle -= 1
        elif c == '(':
            depth_paren += 1
        elif c == ')':
            depth_paren -= 1
        elif c == '[':
            depth_bracket += 1
        elif c == ']':
            depth_bracket -= 1
        elif c == ';' and depth_angle == 0 and depth_paren == 0 and depth_bracket == 0:
            dest = content[start:p].strip()
            return dest, p + 1  # skip the ';'
        p += 1

    return None, pos


def fix_transmute_in_content(content: str) -> str:
    needle = '::core::mem::transmute('
    result = []
    i = 0

    while True:
        pos = content.find(needle, i)
        if pos == -1:
            result.append(content[i:])
            break

        # Skip already-annotated transmute::<...>(
        # Check for '::' before the needle matching '::transmute::<'
        if content[pos-4:pos] in ('e::<', '::<_') or '::transmute::<' in content[max(0,pos-15):pos+len(needle)]:
            result.append(content[i:pos + len(needle)])
            i = pos + len(needle)
            continue

        result.append(content[i:pos])

        inner_start = pos + len(needle)
        inner_end = extract_balanced_parens(content, inner_start)
        inner = content[inner_start:inner_end - 1]

        dest_type, end_pos = extract_dest_type(content, inner_end)

        if dest_type is not None:
            result.append(
                f'::core::mem::transmute::<_, {dest_type}>({inner});'
            )
            i = end_pos
        else:
            # No matching `as TYPE;` - leave unchanged
            result.append(needle + inner + ')')
            i = inner_end

    return ''.join(result)


def fix_file(path: str) -> int:
    with open(path, 'r') as f:
        content = f.read()

    original = content
    content = fix_transmute_in_content(content)

    if content != original:
        with open(path, 'w') as f:
            f.write(content)
        return 1
    return 0


def main(src_dir: str) -> None:
    rs_files = glob.glob(os.path.join(src_dir, '*.rs'))
    fixed = sum(fix_file(f) for f in rs_files)
    print(f"fix_transmute: fixed {fixed}/{len(rs_files)} files")


if __name__ == '__main__':
    src_dir = sys.argv[1] if len(sys.argv) > 1 else os.path.join(
        os.environ.get('OUTPUT_DIR', '/c2rust-projects/projects/minimal'), 'src')
    main(src_dir)
