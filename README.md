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

### Claude porting c2rust produced unsafe rust code to safe rust
```bash
touch $(pwd)/.claude.local.json && \
mkdir $(pwd)/.credentials -p && \
docker run -it --rm \
    --user 1000:1000 \
    -e PORTING_FUNCS="sqlite3SelectNew" \
    -v $(pwd)/.credentials:/home/node/.claude:Z \
    -v $(pwd)/.claude.local.json:/home/node/.claude.json:Z \
    -v $(pwd)/context_entrypoint.sh:/usr/local/bin/context_entrypoint.sh:Z \
    -v $(pwd)/claude-plugin:/plugin:Z \
    -v $(pwd)/crust-sqlite:/workspace:Z \
    layered-sqlite-crust /usr/local/bin/context_entrypoint.sh
```

### Claude working on pattern plugins converting c2rust to rust
```bash
touch $(pwd)/.claude.local.json && \
mkdir $(pwd)/.credentials -p && \
docker run -it --rm \
    --user 1000:1000 \
    -v $(pwd)/.credentials:/home/node/.claude:Z \
    -v $(pwd)/.claude.local.json:/home/node/.claude.json:Z \
    -v $(pwd)/unsafe_rust_fixer:/usr/local/bin/fixer-scripts:Z \
    -v $(pwd)/claude-plugin:/plugin:Z \
    -v $(pwd)/crust-sqlite:/workspace:Z \
    layered-sqlite-crust /usr/local/bin/fixer-scripts/fixer-entrypoint.sh
```
