#!/bin/bash

# write to file ~/.bashrc
cat >> ~/.bashrc << 'EOF'
source /unsafe_rust_fixer/.venv/bin/activate
EOF

source /unsafe_rust_fixer/.venv/bin/activate
cp /docker-scripts/patterns/create-patterns-venv.sh ~/create-venv-docker.sh
chmod +x ~/create-venv-docker.sh

mkdir -p ~/.claude

# assign default value if file is empty
[ -s "$HOME/.claude.json" ] || printf '{}\n' > "$HOME/.claude.json"

PATH="$(python -c 'import sys; sys.path.insert(0, "/plugin"); from config import PATH; print(PATH)'):$PATH"
export PATH="/unsafe_rust_fixer:$PATH"

# just expose the same script as porting docker
ln -sf /workspace/build_all.sh ~/.local/bin/test-sqlite
