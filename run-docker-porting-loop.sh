#!/bin/bash

# Process all JSON files from context-full in parallel
# Usage: ./run-docker-porting-loop.sh [num_parallel_jobs] [file_pattern]
# Examples:
#   ./run-docker-porting-loop.sh              # Run with 0 parallel jobs
#   ./run-docker-porting-loop.sh 8            # Run with 8 parallel jobs
#   ./run-docker-porting-loop.sh 2 "*.json"   # Run with 2 parallel jobs matching pattern

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PARALLEL_JOBS="${1:-0}"
FILE_PATTERN="${2:-*.json}"
RS_NUMBER_FILTER="${3:-}"  # Optional: filter by rs-number (e.g., "0" for -rs-0-)
CONTEXT_FULL_DIR="${SCRIPT_DIR}/context-full"
BASE_SCRIPT="${SCRIPT_DIR}/run-docker-porting.sh"

# Validate prerequisites
if [ ! -d "$CONTEXT_FULL_DIR" ]; then
    echo "Error: context-full directory not found at $CONTEXT_FULL_DIR" >&2
    exit 1
fi

if [ ! -f "$BASE_SCRIPT" ]; then
    echo "Error: run-docker-porting.sh not found at $BASE_SCRIPT" >&2
    exit 1
fi

# Function to process a single JSON file
process_json_file() {
    local json_file="$1"
    local filename=$(basename "$json_file" .json)

    # Extract function name from filename (part after the last dash)
    # e.g., "src-ext-fts3-fts3-rs-1-sqlite3Fts3GetVarint32" -> "sqlite3Fts3GetVarint32"
    local porting_func="${filename##*-}"

    echo "[$(date +'%Y-%m-%d %H:%M:%S')] Processing: $filename → PORTING_FUNCS=$porting_func"

    # Run the base docker script with PORTING_FUNCS set to the extracted function name
    if PORTING_FUNCS="$porting_func" "$BASE_SCRIPT"; then
        echo "[$(date +'%Y-%m-%d %H:%M:%S')] ✓ Completed: $filename"
        return 0
    else
        echo "[$(date +'%Y-%m-%d %H:%M:%S')] ✗ Failed: $filename" >&2
        return 1
    fi
}

export -f process_json_file
export BASE_SCRIPT

# Find all JSON files matching the pattern
echo "Found JSON files to process:"
find "$CONTEXT_FULL_DIR" -maxdepth 1 -name "$FILE_PATTERN" -type f | sort

echo ""
echo "Starting parallel processing with $PARALLEL_JOBS jobs..."
echo ""

# Process files in parallel using xargs
# Each job gets one JSON file path as input
find "$CONTEXT_FULL_DIR" -maxdepth 1 -name "$FILE_PATTERN" -type f | sort | \
    xargs -P "$PARALLEL_JOBS" -I {} bash -c 'process_json_file "$@"' _ {}

echo ""
echo "All files processed!"
