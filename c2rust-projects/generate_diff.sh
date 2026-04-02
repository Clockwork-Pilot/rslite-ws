#!/bin/bash

# Generate file-by-file diffs for Rust projects (folders with Cargo.toml)
# Ignores: target/ and Cargo.lock
# Writes output to both stdout and a file
# Optionally wraps diffs with #[cfg(feature = "...")] annotations

set -e

SEARCH_DIR="${1:-.}"
OUTPUT_FILE="${2:-diffs_$(date +%Y%m%d_%H%M%S).txt}"
FEATURE_NAME="${3:-}"

{
    echo "Searching for Rust projects in: $SEARCH_DIR"
    echo "Generated: $(date)"
    echo "---"

    # Find all Cargo.toml files
    while IFS= read -r cargo_file; do
        project_dir=$(dirname "$cargo_file")

        echo ""
        echo "=== Project: $project_dir ==="

        # Get list of changed files (excluding target/ and Cargo.lock)
        changed_files=$(git -C "$project_dir" diff --name-only HEAD -- ':!target/' ':!Cargo.lock' 2>/dev/null || echo "")

        if [ -z "$changed_files" ]; then
            echo "No changes"
            continue
        fi

        # Process each changed file
        while IFS= read -r file; do
            echo ""
            echo "--- File: $file ---"
            if [ -n "$FEATURE_NAME" ]; then
                echo "#[cfg(feature = \"$FEATURE_NAME\")]"
            fi
            echo "{"
            git -C "$project_dir" diff HEAD -- "$file" 2>/dev/null || echo "Error reading diff for $file"
            echo "}"
        done <<< "$changed_files"

    done < <(find "$SEARCH_DIR" -name "Cargo.toml" -type f)

    echo ""
    echo "---"
    echo "End of diffs"
} | tee "$OUTPUT_FILE"

echo ""
echo "Diffs saved to: $OUTPUT_FILE"
