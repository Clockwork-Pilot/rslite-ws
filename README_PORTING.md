
### Setup

```bash
cd ra_ap_shell/

cargo build --release

cd .. 

./ra_ap_shell/target/release/ast-rs-shell --context-exports # This requires the latest version. Scripts in crust_to_rust_loop are synced with shell

PORTING_FUNCS=<function-name> ./run-docker-porting.sh
```
### Current status
- Copies the repo inside the docker container. 
- Spawns claude inside of it with all the scripts, however, only one must be accesible by it - `filter_content_by_context.py`. 
    There is a subtle remark we can make - I think claude MAY have access to edit a single file, maybe even with expanded lines if it's write() supports it, because of the way we always know what file exactly is the function in due to pre-seed, also, this saves a lot of potential to avoid mistakes with tree-sitter.
- Allows for creating patches for functions, which is more compact type of output then the repo itself and this patch should only touch a single file. 
