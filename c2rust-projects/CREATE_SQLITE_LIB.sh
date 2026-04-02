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
C2RUST_BIN="${C2RUST_BIN:-/c2rust/target/release/c2rust}"
COMPILE_DIR="${COMPILE_DIR:-/c2rust-projects/compile-c2rust}"
REQUIRED_FILES="$COMPILE_DIR/required-files.txt"
SQLITE_ROOT="${1:-/sqlite}"
DEFINES_FILE="${2:-minimal.txt}"
OUTPUT_DIR="${3:-/c2rust-projects/projects/$(basename "$DEFINES_FILE" .txt)}"
DEFINES_PATH="$COMPILE_DIR/compile-options/$DEFINES_FILE"
PROJ_DIR="$(cd "$(dirname "$0")" && pwd)"

echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}C2Rust SQLite Library Creation Script${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo ""

# ============================================================================
# Step 1: Verify Prerequisites
# ============================================================================
echo -e "${YELLOW}[1/7] Verifying prerequisites...${NC}"

if [ ! -f "$C2RUST_BIN" ]; then
    echo -e "${RED}ERROR: C2Rust binary not found at $C2RUST_BIN${NC}"
    echo "Build c2rust with: cd /c2rust && cargo build --release"
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

# Generate mod.rs that declares all transpiled modules (inline in lib.rs)
# Each C source file becomes a module based on its filename
cat > "$OUTPUT_DIR/src/lib.rs" << 'RUST'
#![feature(extern_types, c_variadic)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(warnings)]

extern crate c2rust_bitfields;

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
# Step 6: Integration Setup
# ============================================================================
echo -e "${YELLOW}[6/7] Setting up integration...${NC}"

echo "  ✓ Library configuration ready"
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
