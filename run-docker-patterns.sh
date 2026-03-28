#!/bin/bash


CLAUDE_LOCAL_JSON="$(pwd)/docker-claude-artifacts-c2rust-patterns/.claude.local.json"
CLAUDE_CREDENTIALS_DIR="$(pwd)/docker-claude-artifacts-c2rust-patterns/.credentials"
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

ENTRYPOINT_SCRIPT=$(cat <<EOF


mkdir -p ~/.claude

# assign default value if file is empty
[ -s "\$HOME/.claude.json" ] || printf '{}\n' > "\$HOME/.claude.json"

export PATH="\$(python3 -c 'import sys; sys.path.insert(0, "/plugin"); from config import PATH; print(PATH)'):/unsafe_rust_fixer:\$PATH"
echo "export PATH=\"\$PATH\"" >> ~/.bashrc

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

source /unsafe_rust_fixer/.venv/bin/activate
echo 'source /unsafe_rust_fixer/.venv/bin/activate' >> ~/.bashrc

$ENTRYPOINT_CMD
EOF
)

CMD=(bash -c "$ENTRYPOINT_SCRIPT")

docker run -it --rm \
    --user 1000:1000 \
    -e CLAUDE_PROJECT_ROOT=/unsafe_rust_fixer \
    -e CLAUDE_PLUGIN_ROOT=/plugin \
    -e WORKSPACE_ROOT=/workspace \
    -e CLAUDE_FILE_RULES=/config/deny-file-rules.json \
    -v $(pwd)/docker-scripts/patterns-docker-deny-file-rules.json:/config/deny-file-rules.json:ro,Z \
    -v $CLAUDE_CREDENTIALS_DIR:/home/node/.claude:Z \
    -v $CLAUDE_LOCAL_JSON:/home/node/.claude.json:Z \
    -v $(pwd)/unsafe_rust_fixer:/unsafe_rust_fixer:Z \
    -v $(pwd)/ra_ap_shell:/ra_ap_shell:Z \
    -v $(pwd)/claude-plugin:/plugin:ro,Z \
    -v $(pwd)/crust-sqlite:/workspace:Z \
    layered-sqlite-crust "${CMD[@]}"
