# Get
``` bash
# separately clone repo
git clone git@github.com:YuraLitvinov/crust-sqlite.git

# fetch / update submodules: ra_ap_shell, claude-plugin
git submodule update --init --recursive
```

# Preparation

## Preparation steps
```bash
(cd ra_ap_shell && cargo build)
time (cd crust-sqlite && ../ra_ap_shell/target/debug/ast-rs-shell --context-exports)
du -sh context-full/
```

### Preparation output
Analyzing entire project → ast.json
Initializing rust-analyzer...
  ra: discovering sysroot
  ra: querying project metadata
Loaded 2 local crate(s). Building AST...
Done — 2 crate(s) processed.
✓ Wrote 4550 export context(s) to context-full/

real	27m56,233s
user	27m44,414s
sys	0m10,799s

19G	context-full/

# Next steps
```bash
WORK_DIR=~/git/sqlite-rust-port/crust-sqlite PORTING_FUNCS=sqlite3SelectNew ./filter_content_by_context.py crust-sqlite/src/src/select.rs > /tmp/src_src_select.rs
```
Error: No JSON files found in /home/yaroslav/git/sqlite-rust-port/crust-sqlite

## Logic
We introduce scripts (either bash or python)
```
filter_by_context.py <filename> <func name> <level>
update_fn.py  <filename> <func name> (<tmp-file-path-with-entire-function-text> OR <payload as is>)
```

## Run in Docker:

```bash
docker build -t layered-sqlite-crust .
```

# Claude porting c2rust produced unsafe rust code to safe rust

It is corresponding to `~/.claude/settings.json` inside docker container.
Set permisisons manually in file:
`docker-claude-artifacts-c2rust-port/.claude/settings.json`: 

``` json
  "permissions": {
      "deny": [
        "WebSearch",
        "Explore",
     ],
    "allow": [
      "Read($PORTING_FILE)",
      "Write($PORTING_FILE)",
      "Edit($PORTING_FILE)"
    ]
  }    
```

```bash
PORTING_FUNCS="sqlite3SelectNew" ./run-docker-porting.sh
```

# Claude working on pattern plugins converting c2rust to rust
```bash

#Create venv first
./run-docker-patterns.sh ~/create-venv-docker.sh

# Then use docker as usual
./run-docker-patterns.sh

# Test it
./run-docker-patterns.sh "unsafe-rust-fixer.py --match-patterns='*' --fix src/ && ./build_all.sh && unsafe-rust-fixer.py --match-patterns='*' --fix --dry-run src/"
```
## Permissions within Docker

It is corresponding to `~/.claude/settings.json` inside docker container.
Set permisisons manually in file:
`docker-claude-artifacts-c2rust-patterns/.credentials/settings.json`: 

```json
{
  "permissions": {
    "deny": [
      "Bash(git commit:*)",
      "Bash(git push:*)",
      "Bash(git config:*)",
      "Bash(git log:*)",
      "Bash(gh:*)",
      "Agent(Explore)",
      "Write(/workspace/*)",
      "Edit(/workspace/*)"
    ]
  }
}
```

## Run ast-rs-shell in docker
```bash

# Example of integrated solution
PORTING_FUNCS=fts3BinCompare ./run-docker-porting.sh cat /workspace/src/ext/fts3/fts3_hash.rs

./run-docker-patterns.sh "(cd /ra_ap_shell && cargo build --release)"

# run against sqlite rust codebase
./run-docker-patterns.sh "(cd /workspace && /ra_ap_shell/target/release/ast-rs-shell --context-exports)"
```