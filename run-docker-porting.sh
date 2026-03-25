#!/bin/bash


# Reference README_PORTING.md
PORTING_FUNCS=${PORTING_FUNCS:-"sqlite3SelectNew"}

CONTEXT_FULL=${CONTEXT_FULL:-"$(pwd)/context-full"}

CLAUDE_LOCAL_JSON="$(pwd)/docker-claude-artifacts-c2rust-port/.claude.local.json"
CLAUDE_CREDENTIALS_DIR="$(pwd)/docker-claude-artifacts-c2rust-port/.credentials"

mkdir -p $CLAUDE_CREDENTIALS_DIR
[ -s "$CLAUDE_LOCAL_JSON" ] || printf '{}\n' > "$CLAUDE_LOCAL_JSON"

if [ $# -gt 0 ]; then
    ENTRYPOINT_CMD="$*"
else
    ENTRYPOINT_CMD="claude --plugin-dir /plugin"
fi

ENTRYPOINT_SCRIPT=$(cat <<EOF
export PATH="/usr/local/bin/ra_ap_shell/target/release:\$PATH"

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
[ -s "\$HOME/.claude.json" ] || printf '{}\n' > "\$HOME/.claude.json"

echo 'source /usr/local/bin/ra_ap_shell/.venv/bin/activate' >> ~/.bashrc
source /usr/local/bin/ra_ap_shell/.venv/bin/activate
$ENTRYPOINT_CMD
EOF
)

CMD=(bash -c "$ENTRYPOINT_SCRIPT")

docker run -it --rm \
    --user 1000:1000 \
    -e PORTING_FUNCS \
    -v $CLAUDE_CREDENTIALS_DIR:/home/node/.claude:Z \
    -v $CLAUDE_LOCAL_JSON:/home/node/.claude.json:Z \
    -v $(pwd)/ra_ap_shell/:/usr/local/bin/ra_ap_shell:ro,Z \
    -v $(pwd)/claude-plugin:/plugin:ro,Z \
    -v $CONTEXT_FULL:/workspace/context-full:ro,Z \
    -v $(pwd)/crust_to_rust_loop:/workspace/scripts:ro,Z \
    -v $(pwd)/crust-sqlite:/workspace:Z \
    layered-sqlite-crust "${CMD[@]}"
