#!/usr/bin/env bash
################################################################################
# CREATE_C2RUST_SHELL.sh
# 
# Automates the creation of a C2Rust transpiled SQLite shell from shell.c
# 
# Usage:
#   ./CREATE_C2RUST_SHELL.sh [shell_c_source] [output_dir] [defines_shell_path]
#
# Example:
#   ./CREATE_C2RUST_SHELL.sh /sqlite/shell.c ./crust-sqlite-shell ./defines_shell.txt
#
# Prerequisites:
#   - C2Rust binary: /c2rust/target/release/c2rust
#   - Cargo (Rust compiler)
#   - Existing sqlite_noamalgam Rust library
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
SHELL_SRC="${1:-.sqlite-shell-src/shell.c}"
OUTPUT_DIR="${2:-./crust-sqlite-shell}"
DEFINES_SHELL="${3:-$(cd "$(dirname "$0")" && pwd)/defines_shell.txt}"
PROJ_DIR="$(cd "$(dirname "$0")" && pwd)"

echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}C2Rust SQLite Shell Creation Script${NC}"
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

if [ ! -f "$SHELL_SRC" ]; then
    echo -e "${RED}ERROR: shell.c not found at $SHELL_SRC${NC}"
    exit 1
fi
SHELL_LINES=$(wc -l < "$SHELL_SRC")
echo "  ✓ shell.c found: $SHELL_LINES lines"

if [ ! -f "$DEFINES_SHELL" ]; then
    echo -e "${RED}ERROR: defines_shell.txt not found at $DEFINES_SHELL${NC}"
    exit 1
fi
echo "  ✓ defines_shell.txt found: $DEFINES_SHELL"

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
mkdir -p "$OUTPUT_DIR/tests/fixtures"
echo "  ✓ Created $OUTPUT_DIR"

echo ""

# ============================================================================
# Step 3: Generate C2Rust Configuration
# ============================================================================
echo -e "${YELLOW}[3/7] Generating C2Rust configuration...${NC}"

# Read compilation flags
mapfile -t FLAGS < <(sed 's/\r//' "$DEFINES_SHELL" | grep -v '^$')

# Create c2rust config
cat > "$PROJ_DIR/c2rust-shell.toml" << 'TOML'
[transpilation]
# Emit Rust code that compiles
emit_modules = true
emit_extern_crates = true
use_std = true

# Paths
source_files = ["sqlite-shell-src/shell.c"]
output_dir = "crust-sqlite-shell/src/generated/"

# Compilation settings matching defines_shell.txt
TOML

echo "compile_flags = [" >> "$PROJ_DIR/c2rust-shell.toml"
for flag in "${FLAGS[@]}"; do
    echo "  \"$flag\"," >> "$PROJ_DIR/c2rust-shell.toml"
done
echo "  \"-I.\"," >> "$PROJ_DIR/c2rust-shell.toml"
echo "  \"-I./sqlite-shell-src\"," >> "$PROJ_DIR/c2rust-shell.toml"
echo "]" >> "$PROJ_DIR/c2rust-shell.toml"

cat >> "$PROJ_DIR/c2rust-shell.toml" << 'TOML'

[c2rust.options]
simplify_extern_crates = true
preserve_comments = true
emit_panic_on_fail = true
TOML

echo "  ✓ Generated c2rust-shell.toml"
echo ""

# ============================================================================
# Step 4: Transpile shell.c to Rust
# ============================================================================
echo -e "${YELLOW}[4/7] Transpiling shell.c to Rust...${NC}"
echo "  Running: $C2RUST_BIN transpile $SHELL_SRC"

if "$C2RUST_BIN" transpile "$SHELL_SRC" \
    --emit-modules \
    --disable-rustfmt \
    -- \
    $(printf '%s ' "${FLAGS[@]}") \
    -I. -I./sqlite-shell-src > /tmp/c2rust-transpile.log 2>&1; then
    
    echo "  ✓ Transpilation successful"
    
    # Check output file
    if [ -f "sqlite-shell-src/shell.rs" ]; then
        RUST_LINES=$(wc -l < "sqlite-shell-src/shell.rs")
        echo "  ✓ Generated shell.rs: $RUST_LINES lines"
        
        # Copy to output directory
        cp "sqlite-shell-src/shell.rs" "$OUTPUT_DIR/src/"
        echo "  ✓ Copied to $OUTPUT_DIR/src/shell.rs"
    else
        echo -e "${RED}ERROR: Transpilation succeeded but shell.rs not generated${NC}"
        cat /tmp/c2rust-transpile.log
        exit 1
    fi
else
    echo -e "${RED}ERROR: Transpilation failed${NC}"
    cat /tmp/c2rust-transpile.log
    exit 1
fi

echo ""

# ============================================================================
# Step 5: Create Cargo Configuration
# ============================================================================
echo -e "${YELLOW}[5/7] Creating Cargo configuration...${NC}"

cat > "$OUTPUT_DIR/Cargo.toml" << 'TOML'
[package]
name = "crust-sqlite-shell"
version = "0.1.0"
edition = "2021"
authors = ["C2Rust Automated Migration"]
build = "build.rs"

[[bin]]
name = "sqlite3"
path = "src/main.rs"

[dependencies]
sqlite_noamalgam = { path = "../" }
libc = "0.2"
lazy_static = "1.4"
parking_lot = "0.12"

[build-dependencies]

[dev-dependencies]
assert_cmd = "2.0"
predicates = "3.0"

[profile.release]
lto = true
codegen-units = 1
opt-level = 3
TOML

echo "  ✓ Created $OUTPUT_DIR/Cargo.toml"

cat > "$OUTPUT_DIR/build.rs" << 'RUST'
use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.parent().unwrap();
    let lib_path = workspace_root.join("sqlite-shell/release");

    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=sqlite_noamalgam");
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_path.display());

    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=z");
}
RUST

echo "  ✓ Created $OUTPUT_DIR/build.rs"

cat > "$OUTPUT_DIR/src/main.rs" << 'RUST'
#![feature(extern_types, c_variadic)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(warnings)]

mod shell;

fn main() {
    shell::main();
}
RUST

echo "  ✓ Created $OUTPUT_DIR/src/main.rs"

echo ""

# ============================================================================
# Step 6: Fix Generated Code Issues
# ============================================================================
echo -e "${YELLOW}[6/7] Fixing generated code issues...${NC}"

SHELL_RS="$OUTPUT_DIR/src/shell.rs"

# Fix 1: Add missing seriesCeil/seriesFloor functions
if ! grep -q "fn seriesCeil" "$SHELL_RS"; then
    echo "  Adding seriesCeil and seriesFloor functions..."
    sed -i '/^unsafe extern "C" fn seriesOpen/i\
#[inline]\
fn seriesCeil(r: f64) -> f64 {\
    r.ceil()\
}\
\
#[inline]\
fn seriesFloor(r: f64) -> f64 {\
    r.floor()\
}\
' "$SHELL_RS"
    echo "  ✓ Added series functions"
fi

# Fix 2: Replace large literal that overflows i32
if grep -q "3389603744" "$SHELL_RS"; then
    echo "  Fixing overflow literal (0xca093fa0)..."
    sed -i 's/3389603744/-905363552/g' "$SHELL_RS"
    echo "  ✓ Fixed overflow literal"
fi

echo ""

# ============================================================================
# Step 7: Integration Setup
# ============================================================================
echo -e "${YELLOW}[7/7] Setting up integration...${NC}"

# Update main Cargo.toml
if ! grep -q "crust-sqlite-shell" "$PROJ_DIR/Cargo.toml"; then
    echo "  Adding crust-sqlite-shell to workspace..."
    sed -i '/members = \[/a\    "crust-sqlite-shell",' "$PROJ_DIR/Cargo.toml"
    echo "  ✓ Updated $PROJ_DIR/Cargo.toml"
fi

echo "  ✓ Integration ready"
echo ""

# ============================================================================
# Summary
# ============================================================================
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}✓ C2Rust Shell Creation Complete!${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo ""
echo "Created Files:"
echo "  ✓ $OUTPUT_DIR/Cargo.toml"
echo "  ✓ $OUTPUT_DIR/build.rs"
echo "  ✓ $OUTPUT_DIR/src/main.rs"
echo "  ✓ $OUTPUT_DIR/src/shell.rs ($RUST_LINES lines transpiled)"
echo "  ✓ $PROJ_DIR/c2rust-shell.toml (configuration)"
echo ""
echo "Next Steps:"
echo "  1. Review the transpiled code:"
echo "     less $OUTPUT_DIR/src/shell.rs"
echo ""
echo "  2. Build the shell:"
echo "     cargo build --release -p crust-sqlite-shell"
echo ""
echo "  3. Run the full build pipeline:"
echo "     ./build_all.sh"
echo ""
echo "  4. Test the binary:"
echo "     /sqlite/sqlite3 :memory: \"SELECT sqlite_version();\""
echo ""
echo "Documentation:"
echo "  - $PROJ_DIR/CRUST_SHELL_INTEGRATION.md"
echo "  - $PROJ_DIR/c2rust_shell_actions_log.md"
echo ""

