#!/usr/bin/env bash
################################################################################
# GENERATE_C2RUST_PROJECTS.sh
#
# Generate cumulative C2Rust projects:
# - 0_flags (no flags)
# - 1_flag_<name> (flag 1 only)
# - 2_flags_<names> (flags 1-2)
# - 3_flags_<names> (flags 1-3)
# ... up to 40_flags (all flags)
#
# Also generates "omit_" variants with features disabled
#
################################################################################

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

SQLITE_SRC="${1:-/sqlite}"
OUTPUT_BASE="${2:-/c2rust-projects}"

echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}SQLite Cumulative Configure Projects${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo ""

# Verify prerequisites
echo -e "${YELLOW}[1/3] Verifying prerequisites...${NC}"
[ -d "$SQLITE_SRC" ] || { echo -e "${RED}ERROR: SQLite source not found${NC}"; exit 1; }
[ -f "$SQLITE_SRC/configure" ] || { echo -e "${RED}ERROR: configure not found${NC}"; exit 1; }
echo "  ✓ All prerequisites OK"
echo ""

# SQLite configure flags (excluding --disable-amalgamation which is always default)
CONFIGURE_FLAGS=(
  "--disable-threadsafe"
  "--with-tempstore"
  "--disable-load-extension"
  "--disable-math"
  "--disable-json"
  "--memsys5"
  "--memsys3"
  "--fts3"
  "--fts4"
  "--fts5"
  "--update-limit"
  "--geopoly"
  "--rtree"
  "--session"
  "--dbpage"
  "--dbstat"
  "--disable-carray"
  "--all"
  "--column-metadata"
  "--disable-tcl"
  "--static-tclsqlite3"
  "--disable-readline"
  "--editline"
  "--icu-collations"
  "--static-cli-shell"
  "--static-shells"
  "--amalgamation-extra-src"
  "--disable-rpath"
  "--soname"
  "--dll-basename"
  "--out-implib"
  "--debug"
  "--scanstatus"
  "--dev"
  "--test-status"
  "--gcov"
  "--linemacros"
  "--dynlink-tools"
  "--asan-fsanitize"
)

TOTAL=${#CONFIGURE_FLAGS[@]}

echo -e "${YELLOW}[2/3] Generating cumulative projects (0 to $TOTAL flags)...${NC}"
echo ""
echo "Generating cumulative configurations:"
echo ""

# Generate 0_flags (base, no flags)
echo -n "[0/$TOTAL] 0_flags (base with --disable-amalgamation)... "
cd "$SQLITE_SRC"
make clean > /dev/null 2>&1 || true

PROJECT_DIR="$OUTPUT_BASE/0_flags"
mkdir -p "$PROJECT_DIR"
CONFIG_LOG="$PROJECT_DIR/configure.log"

if ./configure --dump-defines --disable-amalgamation > "$CONFIG_LOG" 2>&1 && [ -f "./config.defines.txt" ]; then
    cp "./config.defines.txt" "$PROJECT_DIR/defines.txt"

    echo "#!/bin/bash" > "$PROJECT_DIR/configure_invocation.txt"
    echo "# Base configuration (--disable-amalgamation default)" >> "$PROJECT_DIR/configure_invocation.txt"
    echo "./configure --dump-defines --disable-amalgamation" >> "$PROJECT_DIR/configure_invocation.txt"

    mkdir -p "$PROJECT_DIR/sqlite"
    tar -C "$SQLITE_SRC" -cf - . 2>/dev/null | tar -C "$PROJECT_DIR/sqlite" -xf - > /dev/null 2>&1

    if [ -d "$PROJECT_DIR/sqlite/src" ] && [ -f "$PROJECT_DIR/defines.txt" ]; then
        echo -e "${GREEN}✓${NC}"
    fi
fi

# 1 to N_flags (cumulative, always with --disable-amalgamation as base)
CUMULATIVE_FLAGS=("--disable-amalgamation")
for i in $(seq 0 $((TOTAL-1))); do
    flag="${CONFIGURE_FLAGS[$i]}"
    CUMULATIVE_FLAGS+=("$flag")

    FLAG_NUM=$((i + 1))

    printf "${BLUE}[%2d/$((TOTAL))${NC} ${FLAG_NUM}_flag%s... " "$FLAG_NUM" "$([ $FLAG_NUM -gt 1 ] && echo 's' || echo '')"

    cd "$SQLITE_SRC"
    make clean > /dev/null 2>&1 || true

    PROJECT_DIR="$OUTPUT_BASE/${FLAG_NUM}_flags"
    mkdir -p "$PROJECT_DIR"
    CONFIG_LOG="$PROJECT_DIR/configure.log"

    CONFIG_CMD="./configure --dump-defines ${CUMULATIVE_FLAGS[@]}"
    if $CONFIG_CMD > "$CONFIG_LOG" 2>&1 && [ -f "./config.defines.txt" ]; then
        cp "./config.defines.txt" "$PROJECT_DIR/defines.txt"

        echo "#!/bin/bash" > "$PROJECT_DIR/configure_invocation.txt"
        echo "# Cumulative: --disable-amalgamation + $i additional flag(s)" >> "$PROJECT_DIR/configure_invocation.txt"
        echo "$CONFIG_CMD" >> "$PROJECT_DIR/configure_invocation.txt"

        mkdir -p "$PROJECT_DIR/sqlite"
        tar -C "$SQLITE_SRC" -cf - . 2>/dev/null | tar -C "$PROJECT_DIR/sqlite" -xf - > /dev/null 2>&1

        if [ -d "$PROJECT_DIR/sqlite/src" ] && [ -f "$PROJECT_DIR/defines.txt" ]; then
            echo -e "${GREEN}✓${NC}"
        fi
    else
        echo -e "${RED}✗${NC}"
    fi
done

echo ""
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}✓ Generated cumulative projects (0 to $TOTAL flags)${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo ""

echo "Output:"
echo "  Location: $OUTPUT_BASE"
echo "  Naming: N_flags (N = number of flags used cumulatively)"
echo "  Each project contains:"
echo "    - sqlite/ - Full SQLite source code"
echo "    - defines.txt - Configuration defines"
echo "    - configure_invocation.txt - Exact configure command"
echo "    - configure.log - Configure output log"
echo ""
