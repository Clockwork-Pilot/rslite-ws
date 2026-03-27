#!/bin/bash

# This script returns a list of JSON files in context-full/ sorted primarily by numerical sequence
# (e.g., all *-0-* files before *-1-* files) and secondarily alphabetically.

set -euo pipefail

CONTEXT_DIR="context-full"

if [ ! -d "$CONTEXT_DIR" ]; then
    echo "ERROR: Directory $CONTEXT_DIR not found." >&2
    exit 1
fi

# Find all .json files in context-full
# Extract the number between the second-to-last hyphen and the last hyphen
# Sort numerically by this number, then by the full path
find "$CONTEXT_DIR" -maxdepth 1 -name "*.json" | \
    sed -E 's/.*-([0-9]+)-[^-]+\.json$/\1 \0/' | \
    sort -n -k1,1 -k2,2 | \
    cut -d' ' -f2-
