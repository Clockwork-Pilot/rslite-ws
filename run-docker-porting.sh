#!/bin/bash

set -euo pipefail

# Reference README_PORTING.md
PORTING_FUNCS=${PORTING_FUNCS:-"sqlite3SelectNew"}

PORTING_JSON=${PORTING_JSON:-$(find context-full -type f -name "*-${PORTING_FUNCS}.json" | head -n 1)}
if [ -z "${PORTING_JSON:-}" ]; then
    echo "ERROR: No JSON file found for function '$PORTING_FUNCS'"
    exit 1
fi

PORTING_JSON=${PORTING_JSON#context-full/}

# Derive the target file path from the JSON filename
# e.g. src-src-select-rs-149-sqlite3SelectNew.json -> src/src/select.rs
PORTING_FILE=$(echo "$PORTING_JSON" \
    | sed 's/-[0-9]*-[^-]*\.json$//' \
    | sed 's/-\([a-z]*\)$/.\1/' \
    | tr '-' '/')

CONTEXT_FULL=${CONTEXT_FULL:-"$(pwd)/context-full"}

CLAUDE_LOCAL_JSON="$(pwd)/docker-claude-artifacts-c2rust-port/.claude.json"
CLAUDE_CREDENTIALS_DIR="$(pwd)/docker-claude-artifacts-c2rust-port/.credentials"


if [ $# -gt 0 ]; then
    ENTRYPOINT_CMD="$*"
else
    ENTRYPOINT_CMD="claude --plugin-dir /plugin"
fi

ENTRYPOINT_SCRIPT=$(cat <<EOF
export PATH="/usr/local/bin/ra_ap_shell/target/release:\$PATH"

mkdir -p ~/.claude
if [ ! -f ~/.claude/settings.local.json ]; then
    cat > ~/.claude/settings.local.json <<SETTINGS_EOF
{
  "permissions": {
    "deny": [
      "Bash(rm*)",
      "Bash(sudo*)",
      "Bash(git*)",
      "Bash(cargo*)",
      "Bash(find*)",
      "Bash(grep*)",
      "Bash(awk*)"
    ],
    "allow": [
      "Read(/workspace/$PORTING_FILE)",
      "Write(/workspace/$PORTING_FILE)",
      "Bash(./scripts/filter_content_by_context.py)"
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

CONTAINER="crust-session-$$"
PATCH_DIR="$(pwd)/patches"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
PATCH_FILE="$PATCH_DIR/session_${TIMESTAMP}.patch"

mkdir -p "$PATCH_DIR"

cleanup() {
    echo ""
    echo "==> Removing container '$CONTAINER'..."
    docker rm -f "$CONTAINER" 2>/dev/null || true
}
trap cleanup EXIT

echo "==> Target file: $PORTING_FILE"
echo "==> Using JSON: $PORTING_JSON"
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

echo "==> Copying Claude config into container..."

docker exec "$CONTAINER" mkdir -p /home/node/.claude

docker cp "$CLAUDE_CREDENTIALS_DIR/." "$CONTAINER:/home/node/.claude"
docker cp "$CLAUDE_LOCAL_JSON" "$CONTAINER:/home/node/.claude.json"

docker exec "$CONTAINER" chown -R 1000:1000 /home/node/.claude /home/node/.claude.json

echo "==> Copying repo into container..."
docker cp "$(pwd)/crust-sqlite/." "$CONTAINER:/workspace"

echo "==> Launching Claude Code..."
docker exec -it "$CONTAINER" "${CMD[@]}"

echo ""
echo "==> Generating patch..."
docker exec "$CONTAINER" \
    git -C /workspace diff HEAD \
    > "$PATCH_FILE"

if [[ ! -s "$PATCH_FILE" ]]; then
    echo "==> No changes — patch is empty."
    rm -f "$PATCH_FILE"
else
    echo "==> Patch written: $PATCH_FILE ($(wc -l < "$PATCH_FILE") lines)"
fi