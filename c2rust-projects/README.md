rm -r /c2rust-projects/projects/minimal/ && \
    /c2rust-projects/CREATE_SQLITE_LIB.sh && \
    cd /c2rust-projects/projects/minimal/ && \
    cargo build
