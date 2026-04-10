#!/bin/bash

CLAUDE_JSON="$(pwd)/docker-files/.claude.local.json"
CLAUDE_CREDENTIALS_DIR="$(pwd)/docker-files/.credentials"
CARGO_DIR="$(pwd)/docker-files/.cargo"
VENV_DIR="$(pwd)/docker-files/venv"
LOCAL_DIR="$(pwd)/docker-files/.local"
# use default if not provided externally
MODEL=${MODEL:-"claude-haiku-4-5"}

# mount support
mkdir -p $CLAUDE_CREDENTIALS_DIR $CARGO_DIR $VENV_DIR $LOCAL_DIR

# assign default value if file is empty
[ -s "$CLAUDE_JSON" ] || printf '{}\n' > "$CLAUDE_JSON"

if [ $# -gt 0 ]; then
    ENTRYPOINT_CMD="$*"
else
    ENTRYPOINT_CMD="claude --dangerously-skip-permissions --model $MODEL --plugin-dir /plugin"
fi

CMD=(bash -c "source /docker-scripts/work-on-sqlite/user-entrypoint.sh ; $ENTRYPOINT_CMD")

# Example of file with rules specified in CLAUDE_FILE_RULES:
# [
#     { "deny-rule": ["$WORKSPACE_ROOT/**"], "reason": "readonly" },
#     { "whitelist-rule": ["$WORKSPACE_ROOT/$PORTING_FILE"] }
# ]

docker run -it --rm \
    -e CLAUDE_PROJECT_ROOT=/workspace \
    -e CLAUDE_PLUGIN_ROOT=/plugin \
    -e WORKSPACE_ROOT=/workspace \
    -e CLAUDE_FILE_RULES=/docker-scripts/work-on-sqlite/y2-plugin-deny-file-rules.json \
    -e PROXY_WRAPPER_CONFIG=/docker-scripts/work-on-sqlite/proxy_wrapper_config.json \
    -e DISABLE_STOP_HOOK=${DISABLE_STOP_HOOK:-} \
    -v $CARGO_DIR:/home/node/.cargo:Z \
    -v $VENV_DIR:/home/node/venv:Z \
    -v $CLAUDE_CREDENTIALS_DIR:/home/node/.claude:Z \
    -v $CLAUDE_JSON:/home/node/.claude.json:Z \
    -v $LOCAL_DIR:/home/node/.local:Z \
    -v $(pwd)/rslite:/workspace:Z \
    rslite-ws "${CMD[@]}"
