#!/bin/bash

# write to file ~/.bashrc
cat >> ~/.bashrc << 'EOF'
source /venv/bin/activate
EOF

source /venv/bin/activate
cp /docker-scripts/work-on-sqlite/create-venv-docker.sh ~/create-venv-docker.sh

mkdir -p ~/.claude

# assign default value if file is empty
[ -s "$HOME/.claude.json" ] || printf '{}\n' > "$HOME/.claude.json"

PATH="$(python -c 'import sys; sys.path.insert(0, "/plugin"); from config import PATH; print(PATH)'):$PATH"
export PATH="/c2rust/target/release:$PATH"
