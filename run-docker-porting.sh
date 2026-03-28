#!/bin/bash

set -euo pipefail

CLAUDE_LOCAL_JSON="$(pwd)/docker-claude-artifacts-c2rust-port/.claude.json"
CLAUDE_CREDENTIALS_DIR="$(pwd)/docker-claude-artifacts-c2rust-port/.claude"
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

echo "Auto-detected:"

# prepare porting arguments: get corresponding json file, ensude we use just one result
JSON_FILE=$(find ./context-full/ -name "*$PORTING_FUNCS*" | head -1)
echo "JSON_FILE: $JSON_FILE"

# using jq - get json field "file":
PORTING_FILE=$(jq -r '.file' "$JSON_FILE")
echo "PORTING_FILE: $PORTING_FILE"

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

cp /crust_to_rust_loop/CLAUDE.md /workspace

# assign default value if file is empty
[ -s "\$HOME/.claude.json" ] || printf '{}\n' > "\$HOME/.claude.json"

export PATH="\$(python3 -c 'import sys; sys.path.insert(0, "/plugin"); from config import PATH; print(PATH)'):/unsafe_rust_fixer:\$PATH"
export PATH="/ra_ap_shell/target/release:\$PATH"
export PATH="/crust_to_rust_loop:\$PATH"
echo 'export PATH="\$PATH"' >> ~/.bashrc

export WORK_DIR=/

cat > ~/create-venv-docker.sh <<'CREATE_VENV_EOF'
(
    python3 -m venv /ra_ap_shell/.venv &&
    source /ra_ap_shell/.venv/bin/activate &&
    pip install -r /ra_ap_shell/requirements.txt &&
    pip install -r /plugin/knowledge_tool/requirements.txt &&
    pip install -r /plugin/requirements.txt
)
CREATE_VENV_EOF
chmod +x ~/create-venv-docker.sh

source /ra_ap_shell/.venv/bin/activate
echo 'source /ra_ap_shell/.venv/bin/activate' >> ~/.bashrc

$ENTRYPOINT_CMD
EOF
)


CMD=(bash -c "$ENTRYPOINT_SCRIPT")

docker run -it \
    --user 1000:1000 \
    -e PORTING_FUNCS \
    -e PORTING_FILE=porting_file.json \
    -e WORKSPACE_ROOT=/workspace \
    -e CLAUDE_PROJECT_ROOT=/workspace \
    -e CLAUDE_PLUGIN_ROOT=/plugin \
    -v $(pwd)/ra_ap_shell:/ra_ap_shell:Z \
    -v $(pwd)/claude-plugin:/plugin:ro,Z \
    -v $(pwd)/crust-sqlite:/x/y/z:ro,Z \
    -v $(pwd)/context-full/$PORTING_FILE:/porting_file.json:ro,Z \
    -v $(pwd)/crust_to_rust_loop:/crust_to_rust_loop:ro,Z \
    layered-sqlite-crust "${CMD[@]}"