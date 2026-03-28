#!/bin/bash
shopt -s globstar nullglob

# This script running by non-priveleged user, so we assign read-only attrs to created files

FROM_DIR="$1"
TO_DIR="$2"

is_excluded() {
    case "$1" in
        .git/*|.git/|.venv/*|.venv/|target/*|target/|.pytest_cache/*|.pytest_cache/) return 0 ;;
    esac
    return 1
}

process_file() {
    local src_file="$1"
    local rel_path="${src_file#$FROM_DIR/}"

    is_excluded "$rel_path" && return

    if [[ -n "${PORTING_FILE:-}" && "$rel_path" == "$PORTING_FILE" ]]; then
        return
    fi

    local dst_file="$TO_DIR/$rel_path"
    mkdir -p "$(dirname "$dst_file")"

    if ! filter_content_by_context.py "$src_file" > "$dst_file" 2>/dev/null; then
        rm -f "$dst_file"
        return
    fi
    chmod 444 "$dst_file"
}

for src_file in "$FROM_DIR"/**/*.{toml,rs,md,txt,sh}; do
    [ -f "$src_file" ] || continue
    process_file "$src_file" &
done

cp -f $FROM_DIR/$PORTING_FILE $TO_DIR/$PORTING_FILE 2>/dev/null
wait
