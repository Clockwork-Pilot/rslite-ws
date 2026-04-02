#!/bin/bash


CLAUDE_LOCAL_JSON="$(pwd)/docker-files/.claude.local.json"
CLAUDE_CREDENTIALS_DIR="$(pwd)/docker-files/.credentials"
# use default if not provided externally
MODEL=${MODEL:-"claude-haiku-4-5"}

# mount support
mkdir -p $CLAUDE_CREDENTIALS_DIR
[ -s "$CLAUDE_LOCAL_JSON" ] || printf '{}\n' > "$CLAUDE_LOCAL_JSON"

if [ $# -gt 0 ]; then
    ENTRYPOINT_CMD="$*"
else
    ENTRYPOINT_CMD="claude --dangerously-skip-permissions --model $MODEL --plugin-dir /plugin"
fi

CMD=(bash -c "source /docker-scripts/work-on-sqlite/user-entrypoint.sh ; $ENTRYPOINT_CMD")

docker run -it --rm \
    -e CLAUDE_PROJECT_ROOT=/workspace \
    -e CLAUDE_PLUGIN_ROOT=/plugin \
    -e WORKSPACE_ROOT=/workspace \
    -e CLAUDE_FILE_RULES=/config/deny-file-rules.json \
    -e PROXY_WRAPPER_CONFIG=/docker-scripts/work-on-sqlite/proxy_wrapper_config.json \
    -v $CLAUDE_CREDENTIALS_DIR:/home/node/.claude:Z \
    -v $CLAUDE_LOCAL_JSON:/home/node/.claude.json:Z \
    -v $(pwd)/docker-scripts/work-on-sqlite/y2-plugin-deny-file-rules.json:/config/deny-file-rules.json:ro,Z \
    -v $(pwd)/unsafe_rust_fixer:/unsafe_rust_fixer:Z \
    -v $(pwd)/claude-plugin:/plugin:ro,Z \
    -v $(pwd)/crust-sqlite:/workspace:Z \
    -v $(pwd)/c2rust-projects:/c2rust-projects:Z \
    layered-sqlite-crust "${CMD[@]}"
