
### Setup

```bash
cd ra_ap_shell/

cargo build --release

cd .. 

./ra_ap_shell/target/release/ast-rs-shell --context-exports # This requires the latest version. Scripts in crust_to_rust_loop are synced with shell

CONTEXT_FULL=<defaults out to $(pwd/context-full)> PORTING_FUNCS=<this should start with any function having 0 dependencies> ./run-docker-patterns.sh 

 ./run-docker-porting-loop.py                              # Process all files                                                                                                                                     
 ./run-docker-porting-loop.py --rs-number 0               # Only -rs-0- files                                                                                                                                      
 ./run-docker-porting-loop.py --rs-number 0 --jobs 8      # -rs-0- with 8 parallel jobs                                                                                                                            
 ./run-docker-porting-loop.py --pattern "*Fts3*" --jobs 2 # Filter + parallel                                                                                                                                         
 ./run-docker-porting-loop.py --help                       # Show all options
```