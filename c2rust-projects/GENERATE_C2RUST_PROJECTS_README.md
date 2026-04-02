# GENERATE_C2RUST_PROJECTS.sh

Generate cumulative SQLite C2Rust transpilation projects with increasing configure flags.

## Usage

```bash
./GENERATE_C2RUST_PROJECTS.sh [sqlite_src] [output_dir]
```

**Default paths:**
- SQLite source: `/sqlite`
- Output: `/c2rust-projects`

## What It Does

Creates 40 projects with cumulative flag configurations:
- `0_flags` - Base: `./configure --dump-defines --disable-amalgamation`
- `1_flags` - Base + `--disable-threadsafe`
- `2_flags` - Base + `--disable-threadsafe --with-tempstore`
- ... up to `39_flags` (all 39 additional flags)

## Each Project Contains

- `sqlite/` - Complete SQLite source code directory
- `defines.txt` - Configuration defines from `./config.defines.txt`
- `configure_invocation.txt` - Exact configure command used
- `configure.log` - Configure output and errors

## Run Transpilation

```bash
for dir in /c2rust-projects/[0-9]*_flags/; do
  cd "$dir/sqlite"
  /c2rust-projects/CREATE_C2RUST_SHELL.sh shell.c ../output/ ../defines.txt
  cd -
done
```

## Requirements

- SQLite source with `./configure` script
- `/c2rust-projects/compile-options/required-files.txt` (file list for transpilation)
- C2Rust binary
- Cargo/Rust toolchain
