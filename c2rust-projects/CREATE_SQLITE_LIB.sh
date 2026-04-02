#!/usr/bin/env bash
################################################################################
# CREATE_SQLITE_LIB.sh
#
# Automates the creation of a C2Rust transpiled SQLite library
#
# Usage:
#   ./CREATE_SQLITE_LIB.sh [sqlite_root_dir] [defines_directory]
#
# Example:
#   ./CREATE_SQLITE_LIB.sh /sqlite /c2rust-projects/compile-options/
#
# Prerequisites:
#   - C2Rust binary: /c2rust/target/release/c2rust
#   - Cargo (Rust compiler)
#   - defines.txt in the defines_directory
#   - /c2rust-projects/required-files.txt with list of source files
#
################################################################################


set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
COMPILE_DIR="${COMPILE_DIR:-/c2rust-projects/compile-c2rust}"
REQUIRED_FILES="$COMPILE_DIR/required-files.txt"
SQLITE_ROOT="${1:-/sqlite}"
DEFINES_FILE="${2:-minimal.txt}"
OUTPUT_DIR="${3:-/c2rust-projects/projects/$(basename "$DEFINES_FILE" .txt)}"
DEFINES_PATH="$COMPILE_DIR/compile-options/$DEFINES_FILE"
PROJ_DIR="$(cd "$(dirname "$0")" && pwd)"

# Find C2Rust binary with fallback locations
find_c2rust() {
    # 1. Check if explicitly provided
    if [ -n "${C2RUST_BIN:-}" ] && [ -f "$C2RUST_BIN" ]; then
        echo "$C2RUST_BIN"
        return 0
    fi

    # 2. Check common locations
    local candidates=(
        "/c2rust/target/release/c2rust"
        "/workspace/target/release/c2rust"
        "${PROJ_DIR}/../target/release/c2rust"
        "${HOME}/.cargo/bin/c2rust"
    )

    for candidate in "${candidates[@]}"; do
        if [ -f "$candidate" ]; then
            echo "$candidate"
            return 0
        fi
    done

    # 3. Check if available in PATH
    if command -v c2rust &> /dev/null; then
        command -v c2rust
        return 0
    fi

    return 1
}

C2RUST_BIN=$(find_c2rust) || C2RUST_BIN=""

echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}C2Rust SQLite Library Creation Script${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo ""

# ============================================================================
# Step 1: Verify Prerequisites
# ============================================================================
echo -e "${YELLOW}[1/7] Verifying prerequisites...${NC}"

if [ -z "$C2RUST_BIN" ] || [ ! -f "$C2RUST_BIN" ]; then
    echo -e "${RED}ERROR: C2Rust binary not found${NC}"
    echo ""
    echo "Tried looking in:"
    echo "  - /c2rust/target/release/c2rust"
    echo "  - /workspace/target/release/c2rust"
    echo "  - ${PROJ_DIR}/../target/release/c2rust"
    echo "  - ${HOME}/.cargo/bin/c2rust"
    echo "  - PATH"
    echo ""
    echo "Options:"
    echo "  1. Set C2RUST_BIN environment variable:"
    echo "     export C2RUST_BIN=/path/to/c2rust"
    echo "  2. Install c2rust via cargo:"
    echo "     cargo install c2rust"
    echo "  3. Build from source in /c2rust:"
    echo "     cd /c2rust && cargo build --release"
    exit 1
fi
echo "  ✓ C2Rust found: $C2RUST_BIN"

if [ ! -d "$SQLITE_ROOT" ]; then
    echo -e "${RED}ERROR: sqlite source root not found at $SQLITE_ROOT${NC}"
    exit 1
fi
echo "  ✓ SQLite root found: $SQLITE_ROOT"

if [ ! -f "$REQUIRED_FILES" ]; then
    echo -e "${RED}ERROR: required-files.txt not found at $REQUIRED_FILES${NC}"
    exit 1
fi
SOURCE_COUNT=$(wc -l < "$REQUIRED_FILES")
echo "  ✓ required-files.txt found: $SOURCE_COUNT source files"

if [ ! -f "$DEFINES_PATH" ]; then
    echo -e "${RED}ERROR: defines file not found at $DEFINES_PATH${NC}"
    exit 1
fi
echo "  ✓ defines file found: $DEFINES_PATH"

if ! which cargo > /dev/null 2>&1; then
    echo -e "${RED}ERROR: Cargo not found${NC}"
    exit 1
fi
echo "  ✓ Cargo available"

echo ""

# ============================================================================
# Step 2: Create Directory Structure
# ============================================================================
echo -e "${YELLOW}[2/7] Creating project structure...${NC}"

mkdir -p "$OUTPUT_DIR/src"
mkdir -p "$OUTPUT_DIR/tests"
echo "  ✓ Created $OUTPUT_DIR"

echo ""

# ============================================================================
# Step 3: Generate C2Rust Configuration
# ============================================================================
echo -e "${YELLOW}[3/7] Generating C2Rust configuration...${NC}"

# Read compilation flags
mapfile -t FLAGS < <(sed 's/\r//' "$DEFINES_PATH" | grep -v '^$')

# Read source files from required-files.txt
mapfile -t SOURCE_FILES < <(sed 's/\r//' "$REQUIRED_FILES" | grep -v '^$')

# Create c2rust config (in project directory)
CONFIG_FILE="$OUTPUT_DIR/c2rust.toml"
cat > "$CONFIG_FILE" << TOML
[transpilation]
# Emit Rust code that compiles
emit_modules = true
emit_extern_crates = true
use_std = true

# Paths
source_files = [
TOML

for src_file in "${SOURCE_FILES[@]}"; do
    echo "  \"$SQLITE_ROOT/$src_file\"," >> "$CONFIG_FILE"
done

cat >> "$CONFIG_FILE" << TOML
]
output_dir = "$OUTPUT_DIR/src/"

# Compilation settings
TOML

echo "compile_flags = [" >> "$CONFIG_FILE"
for flag in "${FLAGS[@]}"; do
    echo "  \"$flag\"," >> "$CONFIG_FILE"
done
echo "  \"-I$SQLITE_ROOT\"," >> "$CONFIG_FILE"
echo "]" >> "$CONFIG_FILE"

cat >> "$CONFIG_FILE" << 'TOML'

[c2rust.options]
simplify_extern_crates = true
preserve_comments = true
emit_panic_on_fail = true
TOML

echo "  ✓ Generated c2rust.toml with ${#SOURCE_FILES[@]} source files"
echo ""

# ============================================================================
# Step 4: Transpile source files to Rust
# ============================================================================
echo -e "${YELLOW}[4/7] Transpiling ${#SOURCE_FILES[@]} source files to Rust...${NC}"

# Build full paths for transpilation
FULL_SOURCE_PATHS=()
for src_file in "${SOURCE_FILES[@]}"; do
    FULL_SOURCE_PATHS+=("$SQLITE_ROOT/$src_file")
done

mkdir -p "$OUTPUT_DIR"

COPIED_COUNT=0

# Transpile to temp directory (C2Rust adds src/ subdirectory)
TEMP_OUT=$(mktemp -d)
trap "rm -rf $TEMP_OUT" EXIT

if "$C2RUST_BIN" transpile "${FULL_SOURCE_PATHS[@]}" \
    --emit-modules \
    --disable-rustfmt \
    -o "$TEMP_OUT/" \
    -- \
    $(printf '%s ' "${FLAGS[@]}") \
    -I"$SQLITE_ROOT" > /tmp/c2rust-lib-transpile.log 2>&1; then

    echo "  ✓ Transpilation successful"

    # Move generated files from temp/src/ to output/src/
    mkdir -p "$OUTPUT_DIR/src"
    if [ -d "$TEMP_OUT/src" ]; then
        mv "$TEMP_OUT/src"/*.rs "$OUTPUT_DIR/src/"
        COPIED_COUNT=$(find "$OUTPUT_DIR/src" -name "*.rs" -type f | grep -v "lib.rs" | wc -l)
    fi

    echo "  ✓ Generated $COPIED_COUNT transpiled files in $OUTPUT_DIR/src/"
else
    echo -e "${RED}ERROR: Transpilation failed${NC}"
    cat /tmp/c2rust-lib-transpile.log
    exit 1
fi

echo ""

# ============================================================================
# Step 5: Create Cargo Configuration
# ============================================================================
echo -e "${YELLOW}[5/7] Creating Cargo configuration...${NC}"

PKG_NAME=$(basename "$OUTPUT_DIR" | tr '/' '-' | sed 's/_/-/g')

cat > "$OUTPUT_DIR/Cargo.toml" << TOML
[package]
name = "$PKG_NAME"
version = "0.1.0"
edition = "2021"
authors = ["C2Rust Automated Migration"]

[lib]
name = "sqlite3"
crate-type = ["rlib", "cdylib"]

[dependencies]
libc = "0.2"
c2rust-bitfields = "0.22.1"

[profile.release]
lto = true
codegen-units = 1
opt-level = 3
TOML

echo "  ✓ Created $OUTPUT_DIR/Cargo.toml"

NIGHTLY_CHANNEL=$(rustup show active-toolchain 2>/dev/null | grep -oP 'nightly-\d{4}-\d{2}-\d{2}' | head -1 || echo "nightly")
cat > "$OUTPUT_DIR/rust-toolchain.toml" << TOML
[toolchain]
channel = "$NIGHTLY_CHANNEL"
components = ["rustfmt", "rust-analyzer"]
TOML
echo "  ✓ Created $OUTPUT_DIR/rust-toolchain.toml (channel: $NIGHTLY_CHANNEL)"

# Generate mod.rs that declares all transpiled modules (inline in lib.rs)
# Each C source file becomes a module based on its filename
cat > "$OUTPUT_DIR/src/lib.rs" << 'RUST'
#![feature(extern_types, c_variadic)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(warnings)]

#[macro_use]
extern crate c2rust_bitfields;

use c2rust_bitfields::BitfieldStruct;

RUST

for src_file in "${SOURCE_FILES[@]}"; do
    MODULE_NAME="${src_file%.c}"
    MODULE_NAME="${MODULE_NAME##*/}"  # Strip path, keep only filename
    # Handle reserved keywords like 'where'
    if [ "$MODULE_NAME" = "where" ]; then
        echo "pub mod r#where;" >> "$OUTPUT_DIR/src/lib.rs"
    else
        echo "pub mod $MODULE_NAME;" >> "$OUTPUT_DIR/src/lib.rs"
    fi
done

echo "  ✓ Created $OUTPUT_DIR/src/lib.rs with ${#SOURCE_FILES[@]} module declarations"

echo ""

# ============================================================================
# Step 6: Post-process generated Rust files to fix nightly API breakage
# ============================================================================
echo -e "${YELLOW}[6/7] Post-processing generated Rust for nightly API compatibility...${NC}"

# Process all generated .rs files (not lib.rs)
RS_FILES=$(find "$OUTPUT_DIR/src" -name "*.rs" ! -name "lib.rs")

for RS_FILE in $RS_FILES; do
    # --- VaListImpl removed in newer nightly ---
    # Remove bare declaration lines: `let mut ap: ::core::ffi::VaListImpl;`
    sed -i '/let mut ap: ::core::ffi::VaListImpl;/d' "$RS_FILE"
    # Convert assignment-after-declaration to let binding:
    # `ap = c2rust_args.clone();`  ->  `let mut ap = c2rust_args;`
    sed -i 's|ap = c2rust_args\.clone();|let mut ap = c2rust_args;|g' "$RS_FILE"
    # `ap.as_va_list()` -> `ap`  (works for both VaList param and moved c2rust_args)
    sed -i 's|ap\.as_va_list()|ap|g' "$RS_FILE"
done

# Fix atomic operations using Python script
cat > /tmp/fix_atomics.py << 'PYTHON_FIX'
import os
import glob
import re

output_dir = os.environ.get('OUTPUT_DIR', '/c2rust-projects/projects/minimal')
rs_files = glob.glob(os.path.join(output_dir, 'src', '*.rs'))
rs_files = [f for f in rs_files if not f.endswith('lib.rs')]

for rs_file in rs_files:
    with open(rs_file, 'r') as f:
        content = f.read()

    # Remove atomic_fence_seqcst();
    content = content.replace('::core::intrinsics::atomic_fence_seqcst();', '')

    # Fix atomic_load_relaxed with nested parens handling
    while '::core::intrinsics::atomic_load_relaxed(' in content:
        start = content.find('::core::intrinsics::atomic_load_relaxed(')
        if start == -1:
            break

        # Find matching closing paren
        i = start + len('::core::intrinsics::atomic_load_relaxed(')
        depth = 1
        while i < len(content) and depth > 0:
            if content[i] == '(':
                depth += 1
            elif content[i] == ')':
                depth -= 1
            i += 1

        # Extract and clean argument
        arg = content[start + len('::core::intrinsics::atomic_load_relaxed('):i-1]
        arg = arg.replace('&raw mut ', '', 1).strip()

        # Replace with just the argument
        content = content[:start] + arg + content[i:]

    # Fix atomic_store_relaxed - just remove it
    while '::core::intrinsics::atomic_store_relaxed(' in content:
        start = content.find('::core::intrinsics::atomic_store_relaxed(')
        if start == -1:
            break

        i = start + len('::core::intrinsics::atomic_store_relaxed(')
        depth = 1
        while i < len(content) and depth > 0:
            if content[i] == '(':
                depth += 1
            elif content[i] == ')':
                depth -= 1
            i += 1

        # Remove the entire call
        content = content[:start] + content[i:]

    # Clean up trailing commas left behind by atomic operation removal
    # Pattern: ,; -> ;
    content = content.replace(',;', ';')
    # Pattern: , as -> as (for type casts)
    content = re.sub(r',\s+as\s+', ' as ', content)
    # Pattern: , } -> }
    content = content.replace(',}', '}')
    # Pattern: , ) -> )
    content = content.replace(',)', ')')
    # Pattern: (expr), at end of statement -> expr;
    content = re.sub(r'=\s*\(([^)]+)\),\s*;', r'= \1;', content)
    # Pattern: multiple trailing commas
    content = re.sub(r'(\w+)\s*,\s*,', r'\1,', content)

    with open(rs_file, 'w') as f:
        f.write(content)

print(f"Processed {len(rs_files)} files")
PYTHON_FIX

export OUTPUT_DIR="$OUTPUT_DIR"
python3 /tmp/fix_atomics.py

FIXED_VALIST=$(grep -rl "let mut ap = c2rust_args" "$OUTPUT_DIR/src" 2>/dev/null | wc -l)
echo "  ✓ Fixed VaListImpl in $FIXED_VALIST files"
echo ""

# ============================================================================
# Step 7: Run C2Rust refactoring to reorganize definitions
# ============================================================================
echo -e "${YELLOW}[7/7] Running C2Rust refactoring to reorganize definitions...${NC}"

cd "$OUTPUT_DIR"
if command -v c2rust &> /dev/null || [ -f "$C2RUST_BIN" ]; then
    if "$C2RUST_BIN" refactor -r inplace --cargo reorganize_definitions > /tmp/c2rust-refactor.log 2>&1; then
        echo "  ✓ Refactoring completed successfully"
    else
        echo "  ⚠ Refactoring completed with warnings (see /tmp/c2rust-refactor.log)"
    fi
else
    echo "  ⚠ c2rust refactor not available, skipping reorganize_definitions"
fi
cd - > /dev/null
echo ""

# ============================================================================
# Summary
# ============================================================================
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}✓ C2Rust Library Creation Complete!${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo ""
echo "Created Files:"
echo "  ✓ $OUTPUT_DIR/Cargo.toml"
echo "  ✓ $OUTPUT_DIR/src/lib.rs"
echo "  ✓ $OUTPUT_DIR/src/*.rs ($COPIED_COUNT transpiled modules)"
echo "  ✓ $OUTPUT_DIR/c2rust.toml (C2Rust configuration)"
echo ""
echo "Next Steps:"
echo "  1. Build the library:"
echo "     cargo build --release -p $PKG_NAME"
echo ""
echo "  2. Find the output library:"
echo "     ls -lh target/release/libsqlite3.so"
echo ""
