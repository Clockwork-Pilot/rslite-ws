mkdir -p ~/.claude

cp /crust_to_rust_loop/CLAUDE.md $WORKSPACE_ROOT

# assign default value if file is empty
[ -s "$HOME/.claude.json" ] || printf '{}\n' > "$HOME/.claude.json"

PATH="$(python3 -c 'import sys; sys.path.insert(0, "/plugin"); from config import PATH; print(PATH)'):$PATH"
PATH="/unsafe_rust_fixer:$PATH"
PATH="/ra_ap_shell:/ra_ap_shell/target/release:$PATH"
export PATH="/crust_to_rust_loop:$PATH"

export WORK_DIR=/x/y

# loop over files in /x/y directory, for every file run filter_content_by_context <filename> > /workspace/<filename> 
# filename should be full patyh relatively to base dir '/x/y'
/docker-scripts/create-context-mirror.sh /x/y $WORKSPACE_ROOT

cp /docker-scripts/porting/create-porting-venv.sh ~/create-venv-docker.sh
chmod +x ~/create-venv-docker.sh

echo "(cd /x/y && ./build_all.sh)" > ~/xyz
chmod +x ~/xyz
ln -sf ~/xyz ~/.local/bin/test-sqlite

source /ra_ap_shell/.venv/bin/activate

# write to file ~/.bashrc
cat >> ~/.bashrc << 'EOF'
source /ra_ap_shell/.venv/bin/activate
EOF