#!/usr/bin/env python3
"""
fix_void_ptr_indexing.py - Fix array indexing on raw void/opaque pointers.

C2Rust generates:
  (*pX).pData[iOffset as isize as usize]      (pData: *const c_void)
  (*pFile).pMapRegion[offset as isize as usize] (pMapRegion: *mut c_void)
  (*p).aOp[n as isize as usize]               (aOp: *mut VdbeOp)

Rust doesn't allow indexing on raw pointers - must use pointer arithmetic:
  ((*pX).pData as *const u8).wrapping_add(iOffset as usize)

This script fixes only the specific known patterns; it does NOT change
valid array indexing on slices, Vecs, or fixed-size arrays.

Usage:
  python3 fix_void_ptr_indexing.py <src_dir>
"""
import os
import sys
import glob
import re


# Specific field patterns known to be raw pointers (not arrays):
# (field_name, source_cast, dest_cast)
VOID_PTR_FIELDS = [
    # btree.rs: pData is *const c_void
    (r'(\(\*\w+\))\.pData\[(\w+\s+as\s+isize\s+as\s+usize)\]',
     r'\1.pData as *const u8_0', '*const ::core::ffi::c_void'),

    # os_unix.rs: pMapRegion is *mut c_void  (read context)
    (r'(\(\*\w+\))\.pMapRegion\[(\w+\s+as\s+isize\s+as\s+usize)\]',
     r'\1.pMapRegion as *mut u8_0', '*mut ::core::ffi::c_void'),

    # vdbeaux.rs: aOp is *mut VdbeOp, cast to u8 for byte offset
    (r'(\(\*\w+\))\.aOp\[(\w+\s+as\s+isize\s+as\s+usize)\]\s+as\s+\*mut\s+u8_0',
     r'\1.aOp as *mut u8_0', None),   # no trailing cast needed
]



def fix_file(path: str) -> int:
    with open(path, 'r') as f:
        content = f.read()

    original = content

    # pData fix: (*ptr).pData[idx] -> ((*ptr).pData as *const u8_0).wrapping_add(idx) as *const c_void
    content = re.sub(
        r'\((\*\w+)\)\.pData\[([^\]]+)\]',
        r'((*\1).pData as *const u8_0).wrapping_add(\2)',
        content
    )

    # pMapRegion fix: (*ptr).pMapRegion[idx] -> ((*ptr).pMapRegion as *mut u8_0).wrapping_add(idx)
    content = re.sub(
        r'\((\*\w+)\)\.pMapRegion\[([^\]]+)\]',
        r'((*\1).pMapRegion as *mut u8_0).wrapping_add(\2)',
        content
    )

    # aOp fix: (*ptr).aOp[idx] as *mut u8_0 -> ((*ptr).aOp as *mut u8_0).wrapping_add(idx)
    content = re.sub(
        r'\((\*\w+)\)\.aOp\[([^\]]+)\]\s+as\s+\*mut\s+u8_0',
        r'((*\1).aOp as *mut u8_0).wrapping_add(\2)',
        content
    )

    if content != original:
        with open(path, 'w') as f:
            f.write(content)
        return 1
    return 0


def main(src_dir: str) -> None:
    rs_files = glob.glob(os.path.join(src_dir, '*.rs'))
    fixed = sum(fix_file(f) for f in rs_files)
    print(f"fix_void_ptr_indexing: fixed {fixed}/{len(rs_files)} files")


if __name__ == '__main__':
    src_dir = sys.argv[1] if len(sys.argv) > 1 else os.path.join(
        os.environ.get('OUTPUT_DIR', '/c2rust-projects/projects/minimal'), 'src')
    main(src_dir)
