#!/usr/bin/env bash
# try_c2rust_refactor.sh
# Attempt c2rust refactoring on the project. Prints result and exits 0 always
# (refactoring is optional; failure is expected on some codebases).
#
# Usage:
#   OUTPUT_DIR=/path/to/project C2RUST_BIN=/path/to/c2rust ./try_c2rust_refactor.sh
#
# Env vars:
#   OUTPUT_DIR   - project directory (default: /c2rust-projects/projects/minimal)
#   C2RUST_BIN   - path to c2rust binary (default: /c2rust/target/release/c2rust)

OUTPUT_DIR="${OUTPUT_DIR:-/c2rust-projects/projects/minimal}"
C2RUST_BIN="${C2RUST_BIN:-/c2rust/target/release/c2rust}"

if [ ! -f "$C2RUST_BIN" ]; then
    echo "try_c2rust_refactor: C2Rust binary not found at $C2RUST_BIN, skipping"
    exit 0
fi

if [ ! -f "$OUTPUT_DIR/Cargo.toml" ]; then
    echo "try_c2rust_refactor: No Cargo.toml at $OUTPUT_DIR, skipping"
    exit 0
fi

echo "try_c2rust_refactor: Attempting reorganize_definitions in $OUTPUT_DIR ..."
cd "$OUTPUT_DIR"

if timeout 60 "$C2RUST_BIN" refactor -r inplace --cargo reorganize_definitions \
    > /tmp/c2rust-refactor.log 2>&1; then
    echo "try_c2rust_refactor: SUCCESS - refactoring applied"
    exit 0
else
    EXIT_CODE=$?
    echo "try_c2rust_refactor: FAILED (exit $EXIT_CODE) - refactoring not applicable"
    echo "  Log tail:"
    tail -5 /tmp/c2rust-refactor.log | sed 's/^/    /'
    exit 0  # always succeed - refactoring is optional
fi
