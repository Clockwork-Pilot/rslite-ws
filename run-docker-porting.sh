#!/bin/bash

set -euo pipefail


# use default if not provided externally
MODEL=${MODEL:-"claude-haiku-4-5"}

CLAUDE_LOCAL_JSON="$(pwd)/docker-claude-artifacts-c2rust-port/.claude.json"
CLAUDE_CREDENTIALS_DIR="$(pwd)/docker-claude-artifacts-c2rust-port/.claude"
CONTEXT_FULL=${CONTEXT_FULL:-"$(pwd)/context-full"}
PATCH_DIR="$(pwd)/patches"
mkdir -p "$PATCH_DIR"

# User should provide following variables:
# - PORTING_FUNCS - function name to port
# - PORTING_FILE - relative source path where function from PORTING_FUNCS is defined
# - JSON_FILE - corresponding relative json file from full-context/

# if not defined exit with error
if [ -z "${PORTING_FUNCS:-}" ] || [ -z "${PORTING_FILE:-}" ] || [ -z "${JSON_FILE:-}" ]; then
    echo "ERROR: PORTING_FUNCS, PORTING_FILE and JSON_FILE must be defined"
    exit 1
fi

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
echo 'export CLAUDE_PROJECT_ROOT=/workspace' >> ~/.bashrc
echo 'export CLAUDE_PLUGIN_ROOT=/plugin' >> ~/.bashrc
echo 'export TEST_LOG=/workspace/log.txt' >> ~/.bashrc

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

source ~/.bashrc
source /ra_ap_shell/.venv/bin/activate

$ENTRYPOINT_CMD
EOF
)


CMD=(bash -c "$ENTRYPOINT_SCRIPT")

docker run -it \
    --user 1000:1000 \
    -e PORTING_FUNCS \
    -e PORTING_FILE \
    -v $(pwd)/ra_ap_shell:/ra_ap_shell:Z \
    -v $(pwd)/claude-plugin:/plugin:ro,Z \
    -v $(pwd)/$PORTING_FILE:/workspace/porting_file.rs:Z \
    -v $(pwd)/$JSON_FILE:/workspace/porting_file.json:ro,Z \
    -v $(pwd)/crust_to_rust_loop:/crust_to_rust_loop:ro,Z \
    layered-sqlite-crust "${CMD[@]}"