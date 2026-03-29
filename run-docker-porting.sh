#!/bin/bash

set -euo pipefail

CLAUDE_LOCAL_JSON="$(pwd)/docker-claude-artifacts-c2rust-port/.claude.local.json"
CLAUDE_CREDENTIALS_DIR="$(pwd)/docker-claude-artifacts-c2rust-port/.credentials"
# use default if not provided externally
MODEL=${MODEL:-"claude-haiku-4-5"}

# Pipeline require following arguments:
# - PORTING_FUNCS - function name to port
# - PORTING_FILE - relative source path where function from PORTING_FUNCS is defined
# - JSON_FILE - corresponding relative json file from full-context/

# if not defined exit with error
if [ -z "${PORTING_FUNCS:-}" ]; then
    echo "ERROR: PORTING_FUNCS must be defined"
    exit 1
fi

# prepare porting arguments: get corresponding json file, ensude we use just one result
JSON_FILE=$(find ./context-full/ -name "*$PORTING_FUNCS*" | head -1)

# using jq - get json field "file":
export PORTING_FILE=$(jq -r '.file' "$JSON_FILE")
echo "Detected PORTING_FILE: $PORTING_FILE"

# mount support
mkdir -p $CLAUDE_CREDENTIALS_DIR
[ -s "$CLAUDE_LOCAL_JSON" ] || printf '{}\n' > "$CLAUDE_LOCAL_JSON"

if [ $# -gt 0 ]; then
    ENTRYPOINT_CMD="$*"
else
    ENTRYPOINT_CMD="claude --dangerously-skip-permissions --model $MODEL --plugin-dir /plugin"
fi

CMD=(bash -c "source /docker-scripts/porting/user-porting-entrypoint.sh ; $ENTRYPOINT_CMD")

docker run -it \
    -e PORTING_FILE \
    -e CONTEXT_SEED=/context_seed.json \
    -e WORK_DIR=/x/y \
    -e WORKSPACE_ROOT=/workspace \
    -e CLAUDE_PROJECT_ROOT=/ra_ap_shell \
    -e CLAUDE_PLUGIN_ROOT=/plugin \
    -v $CLAUDE_CREDENTIALS_DIR:/home/node/.claude:Z \
    -v $CLAUDE_LOCAL_JSON:/home/node/.claude.json:Z \
    -v $(pwd)/ra_ap_shell:/ra_ap_shell:Z \
    -v $(pwd)/claude-plugin:/plugin:ro,Z \
    -v $(pwd)/crust-sqlite:/x/y:Z \
    -v $(pwd)/$JSON_FILE:/context_seed.json:ro,Z \
    -v $(pwd)/crust-sqlite/$PORTING_FILE:/workspace/$PORTING_FILE:rw,Z \
    -v $(pwd)/crust_to_rust_loop:/crust_to_rust_loop:ro,Z \
    layered-sqlite-crust "${CMD[@]}"