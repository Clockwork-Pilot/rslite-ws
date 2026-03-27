#!/usr/bin/env bash
set -euo pipefail

PROJ=/workspace

mkdir -p "$PROJ/sqlite-testfixture" 


CARGO_TARGET_DIR="$PROJ/sqlite-testfixture" cargo build --release -q --features test

bash scripts/testfixture_swap_optimized.sh