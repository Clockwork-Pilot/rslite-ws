# Create projects with enabled compilation flags

## minimal
rm -r /c2rust-projects/projects/minimal/
/c2rust-projects/CREATE_SQLITE_LIB.sh minimal.txt
cd /c2rust-projects/projects/minimal/ && cargo build

## all-features
 /c2rust-projects/CREATE_SQLITE_LIB.sh all-features.txt && cd /c2rust-projects/projects/all-features && cargo build

## all-features-dirty
 /c2rust-projects/CREATE_SQLITE_LIB.sh all-features-dirty.txt && cd /c2rust-projects/projects/all-features-dirty && cargo build 

