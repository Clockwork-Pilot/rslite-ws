#!/usr/bin/env bash
################################################################################
# TRANSPILE_SQLITE_DEFINES.sh
#
# Generates C2Rust transpilations from sqlite's original source code
# using individual defines extracted via ./configure --dump-defines
#
# Usage:
#   ./TRANSPILE_SQLITE_DEFINES.sh [sqlite_src_dir] [output_dir] [target_source]
#
# Example:
#   ./TRANSPILE_SQLITE_DEFINES.sh /sqlite ./c2rust-defines-output shell.c
#
# Prerequisites:
#   - SQLite source code at specified directory with ./configure
#   - C2Rust binary: /c2rust/target/release/c2rust
#   - Cargo (Rust compiler)
#
# Behavior:
#   1. Extracts all defines from ./configure --dump-defines
#   2. Creates defines folder structure (./c2rust-defines-output/defines/)
#   3. For each define, creates a transpilation job
#   4. Runs "make clean" on sqlite source after each transpilation
#
################################################################################

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
C2RUST_BIN="${C2RUST_BIN:-/c2rust/target/release/c2rust}"
SQLITE_SRC="${1:-/sqlite}"
OUTPUT_DIR="${2:-./c2rust-defines-output}"
TARGET_SOURCE="${3:-shell.c}"
PROJ_DIR="$(cd "$(dirname "$0")" && pwd)"

# Derived paths
DEFINES_DIR="$OUTPUT_DIR/defines"
TRANSPILE_JOBS_DIR="$OUTPUT_DIR/transpilation_jobs"
DEFINES_LIST="$OUTPUT_DIR/defines_list.txt"

echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}SQLite Define Extraction & C2Rust Transpilation Script${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo ""

# ============================================================================
# Step 1: Verify Prerequisites
# ============================================================================
echo -e "${YELLOW}[1/6] Verifying prerequisites...${NC}"

if [ ! -f "$C2RUST_BIN" ]; then
    echo -e "${RED}ERROR: C2Rust binary not found at $C2RUST_BIN${NC}"
    echo "Build c2rust with: cd /c2rust && cargo build --release"
    exit 1
fi
echo "  ✓ C2Rust found: $C2RUST_BIN"

if [ ! -d "$SQLITE_SRC" ]; then
    echo -e "${RED}ERROR: SQLite source directory not found at $SQLITE_SRC${NC}"
    exit 1
fi
echo "  ✓ SQLite source found: $SQLITE_SRC"

if [ ! -f "$SQLITE_SRC/configure" ]; then
    echo -e "${RED}ERROR: configure script not found in $SQLITE_SRC${NC}"
    exit 1
fi
echo "  ✓ configure script found"

if [ ! -f "$SQLITE_SRC/$TARGET_SOURCE" ]; then
    echo -e "${RED}ERROR: Target source $TARGET_SOURCE not found in $SQLITE_SRC${NC}"
    exit 1
fi
echo "  ✓ Target source found: $SQLITE_SRC/$TARGET_SOURCE"

if ! which cargo > /dev/null 2>&1; then
    echo -e "${RED}ERROR: Cargo not found${NC}"
    exit 1
fi
echo "  ✓ Cargo available"

echo ""

# ============================================================================
# Step 2: Create Directory Structure
# ============================================================================
echo -e "${YELLOW}[2/6] Creating directory structure...${NC}"

mkdir -p "$DEFINES_DIR"
mkdir -p "$TRANSPILE_JOBS_DIR"
mkdir -p "$OUTPUT_DIR/logs"
mkdir -p "$OUTPUT_DIR/results"

echo "  ✓ Created $OUTPUT_DIR"
echo "  ✓ Created $DEFINES_DIR"
echo "  ✓ Created $TRANSPILE_JOBS_DIR"

echo ""

# ============================================================================
# Step 3: Extract Defines from configure
# ============================================================================
echo -e "${YELLOW}[3/6] Extracting defines from ./configure --dump-defines...${NC}"

pushd "$SQLITE_SRC" > /dev/null

# Run configure with dump-defines and extract define names
if ./configure --dump-defines > "$OUTPUT_DIR/configure_dump.txt" 2>&1; then
    echo "  ✓ Executed ./configure --dump-defines"

    # Parse defines from configure output
    # Format varies, but typically: -DNAME or -DNAME=value
    # Extract just the define names
    grep -oE '\-D[A-Za-z_][A-Za-z0-9_]*' "$OUTPUT_DIR/configure_dump.txt" | \
        sed 's/-D//' | sort -u > "$DEFINES_LIST"

    DEFINE_COUNT=$(wc -l < "$DEFINES_LIST")
    echo "  ✓ Extracted $DEFINE_COUNT unique defines"

    # Show first few defines
    echo "  Sample defines:"
    head -5 "$DEFINES_LIST" | sed 's/^/    - /'
    if [ "$DEFINE_COUNT" -gt 5 ]; then
        echo "    ... and $(($DEFINE_COUNT - 5)) more"
    fi
else
    echo -e "${RED}ERROR: ./configure --dump-defines failed${NC}"
    popd > /dev/null
    exit 1
fi

popd > /dev/null

echo ""

# ============================================================================
# Step 4: Create Individual Define Files
# ============================================================================
echo -e "${YELLOW}[4/6] Creating individual define files...${NC}"

CREATED_COUNT=0
while IFS= read -r define_name; do
    [ -z "$define_name" ] && continue

    # Create a file for each define
    define_file="$DEFINES_DIR/$define_name.txt"
    echo "-D$define_name" > "$define_file"
    ((CREATED_COUNT++))
done < "$DEFINES_LIST"

echo "  ✓ Created $CREATED_COUNT define files in $DEFINES_DIR"
echo ""

# ============================================================================
# Step 5: Create Transpilation Jobs
# ============================================================================
echo -e "${YELLOW}[5/6] Creating transpilation jobs for each define...${NC}"

JOB_COUNT=0
while IFS= read -r define_name; do
    [ -z "$define_name" ] && continue

    JOB_DIR="$TRANSPILE_JOBS_DIR/$define_name"
    mkdir -p "$JOB_DIR"

    # Create job configuration file
    cat > "$JOB_DIR/config.sh" << EOF
#!/usr/bin/env bash
# Auto-generated transpilation job for define: $define_name

DEFINE_NAME="$define_name"
SQLITE_SRC="$SQLITE_SRC"
TARGET_SOURCE="$TARGET_SOURCE"
C2RUST_BIN="$C2RUST_BIN"
JOB_DIR="$JOB_DIR"
OUTPUT_DIR="$OUTPUT_DIR"

# Transpilation function
transpile_with_define() {
    local define="\$1"
    local output_file="\$JOB_DIR/\${define}.rs"
    local log_file="\$JOB_DIR/\${define}.log"

    echo "Transpiling with -D\$define..."

    if "\$C2RUST_BIN" transpile "\$SQLITE_SRC/\$TARGET_SOURCE" \\
        --emit-modules \\
        --disable-rustfmt \\
        -- \\
        -D"\$define" \\
        -I"\$SQLITE_SRC" \\
        -I. > "\$log_file" 2>&1; then

        # Move generated file if it exists
        if [ -f "\$SQLITE_SRC/${TARGET_SOURCE%.c}.rs" ]; then
            mv "\$SQLITE_SRC/${TARGET_SOURCE%.c}.rs" "\$output_file"
            echo "✓ Generated: \$output_file"
            return 0
        else
            echo "⚠ Transpilation succeeded but no output file generated"
            return 1
        fi
    else
        echo "✗ Transpilation failed for -D\$define"
        cat "\$log_file"
        return 1
    fi
}

# Cleanup function
cleanup_sqlite_src() {
    echo "Running 'make clean' in \$SQLITE_SRC..."
    if cd "\$SQLITE_SRC" && make clean > /dev/null 2>&1; then
        echo "✓ make clean completed"
    else
        echo "⚠ make clean failed or not available"
    fi
    cd - > /dev/null
}
EOF
    chmod +x "$JOB_DIR/config.sh"

    ((JOB_COUNT++))
done < "$DEFINES_LIST"

echo "  ✓ Created $JOB_COUNT transpilation job directories"
echo "  ✓ Job configs available in $TRANSPILE_JOBS_DIR/"
echo ""

# ============================================================================
# Step 6: Execute Transpilation Jobs
# ============================================================================
echo -e "${YELLOW}[6/6] Executing transpilation jobs...${NC}"

SUCCESS_COUNT=0
FAIL_COUNT=0
JOB_NUM=0

while IFS= read -r define_name; do
    [ -z "$define_name" ] && continue
    ((JOB_NUM++))

    JOB_DIR="$TRANSPILE_JOBS_DIR/$define_name"
    OUTPUT_FILE="$JOB_DIR/${define_name}.rs"
    LOG_FILE="$JOB_DIR/${define_name}.log"

    echo -ne "${BLUE}[$JOB_NUM/$DEFINE_COUNT]${NC} Transpiling: $define_name... "

    # Execute transpilation
    if "$C2RUST_BIN" transpile "$SQLITE_SRC/$TARGET_SOURCE" \
        --emit-modules \
        --disable-rustfmt \
        -- \
        -D"$define_name" \
        -I"$SQLITE_SRC" \
        -I. > "$LOG_FILE" 2>&1; then

        # Move generated file if it exists
        if [ -f "$SQLITE_SRC/${TARGET_SOURCE%.c}.rs" ]; then
            mv "$SQLITE_SRC/${TARGET_SOURCE%.c}.rs" "$OUTPUT_FILE"
            echo -e "${GREEN}✓${NC}"
            ((SUCCESS_COUNT++))
        else
            echo -e "${RED}✗${NC} (no output)"
            ((FAIL_COUNT++))
        fi
    else
        echo -e "${RED}✗${NC}"
        ((FAIL_COUNT++))
    fi

    # Clean up sqlite source after each transpilation
    if cd "$SQLITE_SRC" && make clean > /dev/null 2>&1; then
        :
    fi
    cd - > /dev/null

done < "$DEFINES_LIST"

echo ""

# ============================================================================
# Summary
# ============================================================================
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}✓ Transpilation Job Batch Complete!${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo ""

echo "Extraction Summary:"
echo "  Defines extracted:      $DEFINE_COUNT"
echo "  Define files created:   $CREATED_COUNT"
echo "  Job directories:        $JOB_COUNT"
echo ""

echo "Transpilation Results:"
echo "  Successful:             $SUCCESS_COUNT"
echo "  Failed:                 $FAIL_COUNT"
echo "  Success rate:           $(( SUCCESS_COUNT * 100 / (SUCCESS_COUNT + FAIL_COUNT) ))%"
echo ""

echo "Output Locations:"
echo "  Defines list:           $DEFINES_LIST"
echo "  Define files:           $DEFINES_DIR/"
echo "  Transpilation jobs:     $TRANSPILE_JOBS_DIR/"
echo "  Transpiled outputs:     $TRANSPILE_JOBS_DIR/<define>/<define>.rs"
echo "  Logs:                   $TRANSPILE_JOBS_DIR/<define>/<define>.log"
echo ""

echo "Next Steps:"
echo "  1. Review transpilation results:"
echo "     ls -la $TRANSPILE_JOBS_DIR/"
echo ""
echo "  2. Check specific transpilation:"
echo "     cat $TRANSPILE_JOBS_DIR/<define_name>/<define_name>.log"
echo ""
echo "  3. Extract Rust files by define:"
echo "     find $TRANSPILE_JOBS_DIR -name '*.rs' -type f"
echo ""
echo "  4. Analyze coverage:"
echo "     echo 'Successful transpilations:'; find $TRANSPILE_JOBS_DIR -name '*.rs' -type f | wc -l"
echo ""
