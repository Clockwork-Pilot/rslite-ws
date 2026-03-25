#!/bin/bash

# Ctrl+Z inside docker run -it sends SIGTSTP to both the container and the host shell,
# which exits back to the host. Running inside tmux prevents this — tmux owns the PTY
# so Ctrl+Z stays inside the session.
# if [ -z "$TMUX" ]; then
#     exec tmux new-session "$0" "$@"
# fi

CLAUDE_LOCAL_JSON="$(pwd)/docker-claude-artifacts-c2rust-patterns/.claude.local.json"
CLAUDE_CREDENTIALS_DIR="$(pwd)/docker-claude-artifacts-c2rust-patterns/.credentials"

mkdir -p $CLAUDE_CREDENTIALS_DIR
[ -s "$CLAUDE_LOCAL_JSON" ] || printf '{}\n' > "$CLAUDE_LOCAL_JSON"

if [ $# -gt 0 ]; then
    ENTRYPOINT_CMD="$*"
else
    ENTRYPOINT_CMD="claude --plugin-dir /plugin"
fi

ENTRYPOINT_SCRIPT=$(cat <<EOF
export PATH="/usr/local/bin/unsafe_rust_fixer:\$PATH"

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
$ENTRYPOINT_CMD
EOF
)

CMD=(bash -c "$ENTRYPOINT_SCRIPT")

docker run -it --rm \
    --user 1000:1000 \
    -v $CLAUDE_CREDENTIALS_DIR:/home/node/.claude:Z \
    -v $CLAUDE_LOCAL_JSON:/home/node/.claude.json:Z \
    -v $(pwd)/unsafe_rust_fixer:/usr/local/bin/unsafe_rust_fixer:Z \
    -v $(pwd)/claude-plugin:/plugin:Z \
    -v $(pwd)/crust-sqlite:/workspace:Z \
    layered-sqlite-crust "${CMD[@]}"
