#!/usr/bin/env bash
################################################################################
# CREATE_SQLITE_LIB.sh
#
# 1. Transpile SQLite C -> Rust via C2Rust
# 2. Create minimal Cargo.toml + lib.rs
# 3. cargo check baseline (raw, no fixes)
# 4. Try c2rust refactor
# 5. cargo check after refactor
# 6. Apply only fixes that are proven necessary (each documented with before/after)
# 7. cargo build
#
# Usage:
#   ./CREATE_SQLITE_LIB.sh [sqlite_root] [defines_file] [output_dir]
################################################################################

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

COMPILE_DIR="${COMPILE_DIR:-/c2rust-projects/compile-c2rust}"
REQUIRED_FILES="$COMPILE_DIR/required-files.txt"
SQLITE_ROOT="${1:-/sqlite}"
DEFINES_FILE="${2:-minimal.txt}"
export OUTPUT_DIR="${3:-/c2rust-projects/projects/$(basename "$DEFINES_FILE" .txt)}"
DEFINES_PATH="$COMPILE_DIR/compile-options/$DEFINES_FILE"
PROJ_DIR="$(cd "$(dirname "$0")" && pwd)"
SCRIPTS_DIR="$PROJ_DIR/create_sqlite_lib"
export SRC_DIR="$OUTPUT_DIR/src"
C2RUST_BIN=`which c2rust`

# Count error[EXXXX] lines from `cargo check` (run in OUTPUT_DIR)
cargo_error_count() {
    local log
    log=$(cd "$OUTPUT_DIR" && cargo check 2>&1 || true)
    echo "$log" | grep -c "^error\[" 2>/dev/null || echo 0
}

echo -e "${GREEN}═══════════════════════════════════════${NC}"
echo -e "${GREEN}C2Rust SQLite Library${NC}"
echo -e "${GREEN}═══════════════════════════════════════${NC}"

# ── Step 1: Prerequisites ────────────────────────────────────────────────────
echo -e "\n${YELLOW}[1] Prerequisites${NC}"
[ -z "$C2RUST_BIN" ] || [ ! -f "$C2RUST_BIN" ] && { echo -e "${RED}ERROR: c2rust not found${NC}"; exit 1; }
[ -d "$SQLITE_ROOT" ]   || { echo -e "${RED}ERROR: $SQLITE_ROOT not found${NC}"; exit 1; }
[ -f "$REQUIRED_FILES" ] || { echo -e "${RED}ERROR: $REQUIRED_FILES not found${NC}"; exit 1; }
[ -f "$DEFINES_PATH" ]  || { echo -e "${RED}ERROR: $DEFINES_PATH not found${NC}"; exit 1; }
command -v cargo >/dev/null 2>&1 || { echo -e "${RED}ERROR: cargo not found${NC}"; exit 1; }
echo "  c2rust: $C2RUST_BIN"

# ── Step 2: Project structure ────────────────────────────────────────────────
echo -e "\n${YELLOW}[2] Creating project structure${NC}"
mkdir -p "$OUTPUT_DIR/src"

# ── Step 3: Transpile ────────────────────────────────────────────────────────
echo -e "\n${YELLOW}[3] Transpiling...${NC}"

mapfile -t FLAGS        < <(sed 's/\r//' "$DEFINES_PATH"    | grep -v '^$')
mapfile -t SOURCE_FILES < <(sed 's/\r//' "$REQUIRED_FILES" | grep -v '^$')

FULL_SOURCE_PATHS=()
for src_file in "${SOURCE_FILES[@]}"; do
    FULL_SOURCE_PATHS+=("$SQLITE_ROOT/$src_file")
done

TEMP_OUT=$(mktemp -d)
trap "rm -rf $TEMP_OUT" EXIT

"$C2RUST_BIN" transpile "${FULL_SOURCE_PATHS[@]}" \
    --emit-modules --disable-rustfmt \
    -o "$TEMP_OUT/" \
    -- $(printf '%s ' "${FLAGS[@]}") -I"$SQLITE_ROOT" \
    > /tmp/c2rust-transpile.log 2>&1 \
    || { echo -e "${RED}Transpilation failed${NC}"; cat /tmp/c2rust-transpile.log; exit 1; }

mkdir -p "$OUTPUT_DIR/src"
[ -d "$TEMP_OUT/src" ] && mv "$TEMP_OUT/src"/*.rs "$OUTPUT_DIR/src/"
COPIED=$(find "$OUTPUT_DIR/src" -name "*.rs" ! -name "lib.rs" | wc -l)
echo "  ✓ $COPIED .rs files"

# ── Step 4: Cargo config ─────────────────────────────────────────────────────
echo -e "\n${YELLOW}[4] Cargo configuration${NC}"

PKG_NAME=$(basename "$OUTPUT_DIR" | tr '/' '-' | sed 's/_/-/g')

cat > "$OUTPUT_DIR/Cargo.toml" << TOML
[package]
name = "$PKG_NAME"
version = "0.1.0"
edition = "2021"
autobins = false

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

NIGHTLY=$(rustup show active-toolchain 2>/dev/null | grep -oP 'nightly-\d{4}-\d{2}-\d{2}' | head -1 || echo "nightly")
cat > "$OUTPUT_DIR/rust-toolchain.toml" << TOML
[toolchain]
channel = "$NIGHTLY"
TOML

{
    printf '#![feature(extern_types, c_variadic)]\n'
    printf '#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused, warnings)]\n\n'
    printf '#[macro_use]\nextern crate c2rust_bitfields;\nuse c2rust_bitfields::BitfieldStruct;\n\n'
    for src_file in "${SOURCE_FILES[@]}"; do
        m="${src_file%.c}"; m="${m##*/}"
        [ "$m" = "where" ] && printf 'pub mod r#where;\n' || printf 'pub mod %s;\n' "$m"
    done
} > "$OUTPUT_DIR/src/lib.rs"

echo "  ✓ Cargo.toml, rust-toolchain.toml, lib.rs"

# ── Step 5: Baseline cargo check (raw, no fixes) ─────────────────────────────
echo -e "\n${YELLOW}[5] Baseline cargo check (no fixes applied)${NC}"
cd "$OUTPUT_DIR"
cargo check 2>&1 | tee /tmp/c2rust-baseline.log || true
BASELINE_ERRORS=$(grep -c "^error\[" /tmp/c2rust-baseline.log || echo 0)
BASELINE_PARSE=$(grep -c "^error:" /tmp/c2rust-baseline.log || echo 0)
echo -e "  errors: ${BASELINE_ERRORS} type-check, ${BASELINE_PARSE} parse"
cd - >/dev/null

# ── Step 6: C2Rust refactor ──────────────────────────────────────────────────
echo -e "\n${YELLOW}[6] Attempting c2rust refactor${NC}"
bash "$SCRIPTS_DIR/try_c2rust_refactor.sh"

echo -e "\n  After refactor:"
cd "$OUTPUT_DIR"
cargo check 2>&1 | tee /tmp/c2rust-post-refactor.log || true
POST_REFACTOR=$(grep -c "^error\[" /tmp/c2rust-post-refactor.log || echo 0)
PARSE_AFTER=$(grep -c "^error:" /tmp/c2rust-post-refactor.log || echo 0)
echo -e "  errors: ${POST_REFACTOR} type-check, ${PARSE_AFTER} parse"
cd - >/dev/null

# ── Step 7: Apply only necessary fixes ───────────────────────────────────────
echo -e "\n${YELLOW}[7] Applying fixes (only those that reduce errors)${NC}"

run_fix() {
    local script="$1"
    local before after delta
    before=$(cargo_error_count); before=${before//[^0-9]/}
    python3 "$SCRIPTS_DIR/$script" "$SRC_DIR"
    after=$(cargo_error_count); after=${after//[^0-9]/}
    delta=$(( before - after ))
    if [ "$delta" -gt 0 ]; then
        echo -e "  ${GREEN}✓ $script: $before -> $after (fixed $delta)${NC}"
    elif [ "$before" -eq 0 ]; then
        echo -e "  ○ $script: already 0 errors, skipped"
    else
        echo -e "  ${YELLOW}~ $script: no change ($before errors)${NC}"
    fi
}

# Only apply fixes that address errors seen in baseline/post-refactor
run_fix fix_bitfield_imports.py
run_fix fix_valist.py
run_fix fix_atomics.py
run_fix fix_stray_commas.py
run_fix fix_match_arms.py
run_fix fix_wal_specific.py
run_fix fix_transmute.py
run_fix fix_void_ptr_indexing.py

# ── Step 8: Final build ──────────────────────────────────────────────────────
echo -e "\n${YELLOW}[8] Final cargo build${NC}"
cd "$OUTPUT_DIR"
if cargo build 2>&1 | tee /tmp/c2rust-build.log; then
    echo -e "\n${GREEN}✓ cargo build succeeded${NC}"
else
    echo -e "\n${RED}✗ cargo build FAILED${NC}"
    echo "  See /tmp/c2rust-build.log"
    exit 1
fi
cd - >/dev/null

echo -e "\n${GREEN}═══════════════════════════════════════${NC}"
echo -e "${GREEN}✓ Done: $OUTPUT_DIR${NC}"
echo -e "${GREEN}═══════════════════════════════════════${NC}"
