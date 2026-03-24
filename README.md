# Get
``` bash
# separately clone repo
git clone git@github.com:YuraLitvinov/crust-sqlite.git

# fetch / update submodules
git submodule update --init --recursive
```

# 
``` bash
git clone git@github.com:YuraLitvinov/ra_ap-shell.git
cd ra_ap-shell && cargo build
./target/debug/ast-rs-shell --context-exports
```

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
    -e PORTING_FUNCS="sqlite3SelectNew,sqlite3_expanded_sql" \
    -v $(pwd)/.credentials:/home/node/.claude:Z \
    -v $(pwd)/.claude.local.json:/home/node/.claude.json:Z \
    -v $(pwd)/context-layer:/usr/local/bin/context-layer:Z \
    -v $(pwd)/claude-plugin:/plugin:Z \
    -v $(pwd)/crust-sqlite:/workspace:Z \
    layered-sqlite-crust /usr/local/bin/context-layer/context_entrypoint.sh
```

### Claude working on pattern plugins converting c2rust to rust
```bash
touch $(pwd)/.claude.local.json && \
mkdir $(pwd)/.credentials -p && \
docker run -it --rm \
    --user 1000:1000 \
    -e PORTING_FUNCS="sqlite3SelectNew,sqlite3_expanded_sql" \
    -v $(pwd)/.credentials:/home/node/.claude:Z \
    -v $(pwd)/.claude.local.json:/home/node/.claude.json:Z \
    -v $(pwd)/scripts:/usr/local/bin/scripts:Z \
    -v $(pwd)/claude-plugin:/plugin:Z \
    -v $(pwd)/crust-sqlite:/workspace:Z \
    layered-sqlite-crust /usr/local/bin/scripts/scripts-entrypoint.sh
```
