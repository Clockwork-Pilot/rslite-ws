#!/bin/bash

source /ra_ap_shell/.venv/bin/activate
cp /docker-scripts/porting/create-porting-venv.sh ~/create-venv-docker.sh
chmod +x ~/create-venv-docker.sh

mkdir -p ~/.claude


# assign default value if file is empty
[ -s "$HOME/.claude.json" ] || printf '{}\n' > "$HOME/.claude.json"



# loop over files in /x/y directory, for every file run filter_content_by_context <filename> > /workspace/<filename> 
# filename should be full patyh relatively to base dir '/x/y'
# /docker-scripts/create-context-mirror.sh /x/y $WORKSPACE_ROOT

rm -f $WORKSPACE_ROOT/CLAUDE.md

cp /crust_to_rust_loop/CLAUDE.md $WORKSPACE_ROOT


echo "cd /workspace && ./build_all.sh" > ~/xyz
chmod +x ~/xyz
ln -sf ~/xyz ~/.local/bin/test-sqlite


# write to file ~/.bashrc
cat >> ~/.bashrc << 'EOF'
source /ra_ap_shell/.venv/bin/activate
EOF