#!/bin/bash

# Ctrl+Z inside docker run -it sends SIGTSTP to both the container and the host shell,
# which exits back to the host. Running inside tmux prevents this — tmux owns the PTY
# so Ctrl+Z stays inside the session.
# if [ -z "$TMUX" ]; then
#     exec tmux new-session "$0" "$@"
# fi

CLAUDE_LOCAL_JSON="$(pwd)/docker-claude-artifacts-c2rust-patterns/.claude.local.json"
CLAUDE_CREDENTIALS_DIR="$(pwd)/docker-claude-artifacts-c2rust-patterns/.credentials"
# use default if not provided externally
MODEL=${MODEL:-"claude-haiku-4-5"}

mkdir -p $CLAUDE_CREDENTIALS_DIR
[ -s "$CLAUDE_LOCAL_JSON" ] || printf '{}\n' > "$CLAUDE_LOCAL_JSON"

if [ $# -gt 0 ]; then
    ENTRYPOINT_CMD="$*"
else
    ENTRYPOINT_CMD="claude --dangerously-skip-permissions --model $MODEL --plugin-dir /plugin"
fi

ENTRYPOINT_SCRIPT=$(cat <<EOF


mkdir -p ~/.claude

# Overwrite it always when setting entrypoint. If you need to edit externally, instead edit: ~/.claude/settings.json
cat > ~/.claude/settings.local.json <<'SETTINGS_EOF'
{
  "permissions": {
    "deny": [
      "Bash(git commit:*)",
      "Bash(git push:*)",
      "Bash(git log:*)",
      "Bash(gh:*)",
      "Agent(Explore)"
    ],
    "allow": [
      "Read(*)",
      "Write(*)"
    ]
  }
}
SETTINGS_EOF

[ -s "\$HOME/.claude.json" ] || printf '{}\n' > "\$HOME/.claude.json"

export PATH="\$(python3 -c 'import sys; sys.path.insert(0, "/plugin"); from config import PATH; print(PATH)'):/unsafe_rust_fixer:\$PATH"
echo 'export PATH="\$PATH"' >> ~/.bashrc
echo 'export CLAUDE_PROJECT_ROOT=/workspace' >> ~/.bashrc
echo 'export CLAUDE_PLUGIN_ROOT=/plugin' >> ~/.bashrc
echo 'export TEST_LOG=/workspace/log.txt' >> ~/.bashrc
echo 'source /unsafe_rust_fixer/.venv/bin/activate' >> ~/.bashrc

cat > ~/create-venv-docker.sh <<'CREATE_VENV_EOF'
(
    python3 -m venv /unsafe_rust_fixer/.venv &&
    source /unsafe_rust_fixer/.venv/bin/activate &&
    pip install -r /unsafe_rust_fixer/requirements.txt &&
    pip install -r /plugin/knowledge_tool/requirements.txt &&
    pip install -r /plugin/requirements.txt
)
CREATE_VENV_EOF
chmod +x ~/create-venv-docker.sh

source ~/.bashrc
source /unsafe_rust_fixer/.venv/bin/activate

$ENTRYPOINT_CMD
EOF
)

CMD=(bash -c "$ENTRYPOINT_SCRIPT")

docker run -it --rm \
    --user 1000:1000 \
    -v $CLAUDE_CREDENTIALS_DIR:/home/node/.claude:Z \
    -v $CLAUDE_LOCAL_JSON:/home/node/.claude.json:Z \
    -v $(pwd)/unsafe_rust_fixer:/unsafe_rust_fixer:Z \
    -v $(pwd)/claude-plugin:/plugin:ro,Z \
    -v $(pwd)/crust-sqlite:/workspace:Z \
    layered-sqlite-crust "${CMD[@]}"
