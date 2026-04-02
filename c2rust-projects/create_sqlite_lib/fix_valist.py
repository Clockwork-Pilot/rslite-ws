#!/usr/bin/env python3
"""Fix VaListImpl API changes for nightly Rust compatibility"""
import os
import glob

def fix_valist(output_dir):
    rs_files = glob.glob(os.path.join(output_dir, 'src', '*.rs'))
    fixed_count = 0

    for rs_file in rs_files:
        try:
            with open(rs_file, 'r') as f:
                content = f.read()

            original = content

            # Remove bare VaListImpl declarations
            content = content.replace('let mut ap: ::core::ffi::VaListImpl;', '')

            # Convert assignment to let binding
            content = content.replace('ap = c2rust_args.clone();', 'let mut ap = c2rust_args;')

            # Replace as_va_list() calls
            content = content.replace('ap.as_va_list()', 'ap')

            if content != original:
                with open(rs_file, 'w') as f:
                    f.write(content)
                fixed_count += 1
        except Exception as e:
            pass

    return fixed_count

if __name__ == '__main__':
    output_dir = os.environ.get('OUTPUT_DIR', '/c2rust-projects/projects/minimal')
    count = fix_valist(output_dir)
    print(f"Fixed VaListImpl in {count} files")
