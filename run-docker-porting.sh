#!/bin/bash

PORTING_FUNCS=${PORTING_FUNCS:-"sqlite3SelectNew"}

CLAUDE_LOCAL_JSON="$(pwd)/docker-claude-artifacts-c2rust-port/.claude.local.json"
CLAUDE_CREDENTIALS_DIR="$(pwd)/docker-claude-artifacts-c2rust-port/.credentials"

mkdir -p $CLAUDE_CREDENTIALS_DIR
[ -s "$CLAUDE_LOCAL_JSON" ] || printf '{}\n' > "$CLAUDE_LOCAL_JSON"

ENTRYPOINT_SCRIPT=$(cat <<'EOF'
mkdir -p ~/.claude
if [ ! -f ~/.claude/settings.local.json ]; then
    cat > ~/.claude/settings.local.json <<'SETTINGS_EOF'
{
  "permissions": {
    "deny": [
      "Bash(rm*)",
      "Bash(sudo*)"
    ],
    "allow": [
      "Read(*)",
      "Write(*)"
    ]
  }
}
SETTINGS_EOF
fi
[ -s "$HOME/.claude.json" ] || printf '{}\n' > "$HOME/.claude.json"
claude --plugin-dir /plugin
EOF
)

if [ $# -gt 0 ]; then
    CMD=(bash -c "$@")
else
    CMD=(bash -c "$ENTRYPOINT_SCRIPT")
fi

docker run -it --rm \
    --user 1000:1000 \
    -e PORTING_FUNCS \
    -v $CLAUDE_CREDENTIALS_DIR:/home/node/.claude:Z \
    -v $CLAUDE_LOCAL_JSON:/home/node/.claude.json:Z \
    -v $(pwd)/claude-plugin:/plugin:Z \
    -v $(pwd)/crust-sqlite:/workspace:Z \
    layered-sqlite-crust "${CMD[@]}"
