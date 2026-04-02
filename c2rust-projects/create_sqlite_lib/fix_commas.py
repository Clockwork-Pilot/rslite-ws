#!/usr/bin/env python3
"""Fix stray comma syntax errors introduced by C2Rust"""
import os
import glob
import re

def fix_commas(output_dir):
    rs_files = glob.glob(os.path.join(output_dir, 'src', '*.rs'))

    for rs_file in rs_files:
        try:
            with open(rs_file, 'r') as f:
                content = f.read()

            # Fix 1: Return statements with trailing commas
            # Pattern: "return expr," -> "return expr;"
            content = re.sub(r'(\breturn\s+[^;,\n]+),\s*$', r'\1;', content, flags=re.MULTILINE)

            # Fix 2: Match arm returns with trailing commas
            # Pattern: "=> return expr," -> "=> return expr;"
            content = re.sub(r'(=>\s+return\s+[^;,\n]+),\s*$', r'\1;', content, flags=re.MULTILINE)

            # Fix 3: If statements with stray commas before operators
            # Pattern: "if (...expr)," followed by operator
            content = re.sub(r'(\bif\b[^{;\n]+),(\s*[!=<>])', r'\1\2', content)

            # Fix 4: If statements ending with comma before opening brace
            # Pattern: "if expr," with { on next line
            lines = content.split('\n')
            result = []
            for i, line in enumerate(lines):
                stripped = line.rstrip()
                if stripped.endswith(',') and i + 1 < len(lines):
                    next_stripped = lines[i + 1].strip()
                    if next_stripped.startswith('{'):
                        line = stripped[:-1] + '\n'
                result.append(line)
            content = '\n'.join(result)

            # Fix 5: Assignments followed by method calls with trailing comma
            # Pattern: "= expr," followed by "."
            content = re.sub(r'(=\s*[^,;\n]+),\s*(\.\w+)', r'\1\2', content)

            with open(rs_file, 'w') as f:
                f.write(content)
        except Exception as e:
            pass

    return len(rs_files)

if __name__ == '__main__':
    output_dir = os.environ.get('OUTPUT_DIR', '/c2rust-projects/projects/minimal')
    count = fix_commas(output_dir)
    print(f"Fixed stray commas in {count} files")
