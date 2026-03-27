#!/bin/bash

set -euo pipefail


CLAUDE_LOCAL_JSON="$(pwd)/docker-claude-artifacts-c2rust-port/.claude.json"
CLAUDE_CREDENTIALS_DIR="$(pwd)/docker-claude-artifacts-c2rust-port/.claude"
CONTEXT_FULL=${CONTEXT_FULL:-"$(pwd)/context-full"}
PATCH_DIR="$(pwd)/patches"
mkdir -p "$PATCH_DIR"


# Determine if we are in loop mode
# If no specific function or JSON is provided via environment, run in loop mode using all sorted files.
if [ -z "${PORTING_FUNCS:-}" ] && [ -z "${PORTING_JSON:-}" ]; then
    echo "==> No specific function provided. Running in LOOP MODE..."
    JSON_FILES=$(./scripts/get_sorted_porting_files.sh)
else
    echo "==> Running in SINGLE FILE MODE..."
    if [ -n "${PORTING_JSON:-}" ]; then
        JSON_FILES="$PORTING_JSON"
    else
        JSON_FILES=$(find context-full -type f -name "*-${PORTING_FUNCS}.json" | head -n 1)
    fi
fi

if [ -z "$JSON_FILES" ]; then
    echo "ERROR: No JSON files found to process."
    exit 1
fi


if [ $# -gt 0 ]; then
    ENTRYPOINT_CMD="$*"
else
    ENTRYPOINT_CMD="claude --plugin-dir /plugin"
fi

for FULL_JSON_PATH in $JSON_FILES; do
    PORTING_JSON=$(basename "$FULL_JSON_PATH")
    PORTING_FUNCS=$(echo "$PORTING_JSON" | sed -E 's/.*-([^-]+)\.json$/\1/')
    
    # Derive the target file path from the JSON filename
    # e.g. src-src-select-rs-149-sqlite3SelectNew.json -> src/src/select.rs
    PORTING_FILE=$(echo "$PORTING_JSON" \
        | sed 's/-[0-9]*-[^-]*\.json$//' \
        | sed 's/-\([a-z]*\)$/.\1/' \
        | tr '-' '/')

    echo "==> Processing: $PORTING_FUNCS"
    echo "==> Target file: $PORTING_FILE"

    CONTAINER="crust-session-$$"
    PATCH_FILE="$PATCH_DIR/session_${PORTING_FUNCS}.patch"

    ENTRYPOINT_SCRIPT=$(cat <<EOF
export PATH="/usr/local/bin/ra_ap_shell/target/release:\$PATH"

mkdir -p /workspace/.claude
if [ ! -f /workspace/.claude/settings.local.json ]; then
    cat > /workspace/.claude/settings.local.json <<SETTINGS_EOF
{                                                                                                                                                                                                                 
    "permissions": {
      "allow": [                                                  
        "Read($PORTING_FILE)",
        "Write($PORTING_FILE)",
        "Edit($PORTING_FILE)",
        "Bash(scripts/filter_content_by_context.py)"                                                                                                                                                                
      ]                                                                                                                                                                                                         
    }                                                                                                                                                                                                              
}      
SETTINGS_EOF
fi

[ -s "\$HOME/.claude.json" ] || printf '{}\n' > "\$HOME/.claude.json"

echo 'source /usr/local/bin/ra_ap_shell/.venv/bin/activate' >> ~/.bashrc
source /usr/local/bin/ra_ap_shell/.venv/bin/activate

$ENTRYPOINT_CMD
EOF
)

    CMD=(bash -c "$ENTRYPOINT_SCRIPT")

    cleanup() {
        docker rm -f "$CONTAINER" 2>/dev/null || true
    }
    trap cleanup EXIT

    echo "==> Starting container '$CONTAINER'..."
    docker run -dit \
        --name "$CONTAINER" \
        --user 1000:1000 \
        -e PORTING_FUNCS \
        -e PORTING_FILE \
        -v "$(pwd)/ra_ap_shell/":/usr/local/bin/ra_ap_shell:ro,Z \
        -v "$(pwd)/claude-plugin":/plugin:ro,Z \
        -v "$CONTEXT_FULL":/context-full:ro,Z \
        -v "$(pwd)/crust_to_rust_loop":/workspace/scripts:ro,Z \
        layered-sqlite-crust sleep inf

    echo "==> Copying Claude config and repo into container..."
    docker exec "$CONTAINER" mkdir -p /home/node/.claude
    docker cp "$CLAUDE_CREDENTIALS_DIR/." "$CONTAINER:/home/node/.claude"
    docker cp "$CLAUDE_LOCAL_JSON" "$CONTAINER:/home/node/.claude.json"
    docker exec "$CONTAINER" chown -R 1000:1000 /home/node/.claude /home/node/.claude.json
    docker cp "$(pwd)/crust-sqlite/." "$CONTAINER:/workspace"

    echo "==> Launching Claude Code..."
    if ! docker exec -it "$CONTAINER" "${CMD[@]}"; then
        echo "==> Claude Code failed for $PORTING_FUNCS. Skipping cycle."
        cleanup
        continue
    fi

    echo "==> Running build and test check..."
    if ! docker exec "$CONTAINER" /workspace/scripts/build_all.sh; then
        echo "==> Build or tests FAILED for $PORTING_FUNCS. Skipping cycle."
        cleanup
        continue
    fi

    echo "==> SUCCESS! Generating patch..."
    docker exec "$CONTAINER" git -C /workspace diff HEAD > "$PATCH_FILE"
    
    if [[ ! -s "$PATCH_FILE" ]]; then
        echo "==> No changes — patch is empty."
        rm -f "$PATCH_FILE"
    else
        echo "==> Patch written: $PATCH_FILE ($(wc -l < "$PATCH_FILE") lines)"
    fi

    echo "==> Removing processed JSON file: $FULL_JSON_PATH"
    rm -f "$FULL_JSON_PATH"

    cleanup
    trap - EXIT
done

echo "==> Loop completed."
