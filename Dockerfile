FROM rust:bookworm

ARG TZ
ENV TZ="$TZ"

ARG CLAUDE_CODE_VERSION=latest
ENV DEVCONTAINER=true

# Install basic development tools and iptables/ipset
RUN apt-get update && apt-get install -y --no-install-recommends \
  less \
  git \
  procps \
  sudo \
  fzf \
  man-db \
  unzip \
  gnupg2 \
  gh \
  iptables \
  ipset \
  iproute2 \
  dnsutils \
  aggregate \
  jq \
  lsb-release \
  nano \
  vim \
  wget \
  curl \
  tcl \
  tcllib \
  tcl8.6-dev \
  gdb \ 
  tcl-dev \
  tcl8.6-tdbc \
  tcl8.6 \
  tcl8.6-tdbc-sqlite3 \
  libicu-dev \
  build-essential \
  && apt-get clean && rm -rf /var/lib/apt/lists/*

RUN apt-get update && apt-get install -y --no-install-recommends \
  python3 \
  python3-pip \
  python3-venv \
  pinentry-curses \
  gosu \
  && apt-get clean && rm -rf /var/lib/apt/lists/* \
  && curl -LsSf https://astral.sh/uv/install.sh | sh  


RUN curl -fsSL https://github.com/tmux/tmux-builds/releases/download/v3.6a/tmux-3.6a-linux-x86_64.tar.gz \
  | tar -xz -C /usr/local/bin tmux


ARG USERNAME=node
ARG HOME=/home/$USERNAME

RUN useradd -m -u 1000 $USERNAME

# Create workspace and config directories
RUN mkdir -p $HOME/ && \
  chown -R $USERNAME:$USERNAME $HOME/

RUN mkdir -p /sqlite && \
  chown -R $USERNAME:$USERNAME /sqlite
  
USER $USERNAME

RUN cd /sqlite && \
    wget https://sqlite.org/2026/sqlite-src-3510200.zip && \
    ls /sqlite && \
    unzip /sqlite/sqlite-src-3510200.zip -d /sqlite && \
    rm /sqlite/sqlite-src-3510200.zip && \
    mv /sqlite/sqlite-src-3510200/* /sqlite/ && \
    rm -rf /sqlite/sqlite-src-3510200 && \
    cd /sqlite && \
    ./configure --all --disable-amalgamation && make && rm *.o

RUN rustup install nightly-2026-03-26-x86_64-unknown-linux-gnu \
    && rustup component add --toolchain nightly-2026-03-26-x86_64-unknown-linux-gnu \
       rustfmt rust-analyzer clippy
      
RUN curl -fsSL https://claude.ai/install.sh | bash

# add claude code path
ENV PATH="$HOME/.local/bin:$PATH"

# add uv path
ENV PATH="/root/.local/bin:$PATH"


USER root

RUN apt-get update && apt-get install -y --no-install-recommends \
    libclang-dev \
    clang \
    llvm-dev \
    cmake \
    libssl-dev \
    pkg-config \
    && apt-get clean && rm -rf /var/lib/apt/lists/*


RUN mkdir -p /c2rust && \
    wget https://github.com/immunant/c2rust/archive/refs/heads/master.zip -O /tmp/c2rust-master.zip && \
    unzip /tmp/c2rust-master.zip -d /c2rust && \
    mv /c2rust/c2rust-master/* /c2rust/ && \
    rm -rf /c2rust/c2rust-master /tmp/c2rust-master.zip

RUN cd /c2rust && cargo fetch --verbose

RUN cd /c2rust && cargo build --release -j6

ENV PATH="/c2rust/target/release:$PATH"

RUN ln -s /usr/include/tcl/tcl.h /usr/include/tcl.h \
	&& ln -s /usr/include/tcl/tclOODecls.h /usr/include/tclOODecls.h \
	&& ln -s /usr/include/tcl/tclPlatDecls.h /usr/include/tclPlatDecls.h \
	&& ln -s /usr/include/tcl/tclDecls.h /usr/include/tclDecls.h \
	&& ln -s /usr/include/tcl/tclTomMath.h /usr/include/tclTomMath.h \
	&& ln -s /usr/include/tcl/tclTomMathDecls.h /usr/include/tclTomMathDecls.h \
	&& ln -s /usr/lib/tclConfig.sh /usr/lib64/tclConfig.sh

RUN usermod -aG tty $USERNAME


COPY docker-scripts /docker-scripts
RUN cp /docker-scripts/docker-entrypoint.sh /usr/local/bin
RUN chmod +x /usr/local/bin/docker-entrypoint.sh

# Install proxy wrapper — intercepts git, gh, cat, sed inside namespaces.
RUN cp /docker-scripts/proxy_wrapper.py /usr/local/bin/proxy_wrapper.py
RUN chmod +x /usr/local/bin/proxy_wrapper.py \
    && ln -sf /usr/local/bin/proxy_wrapper.py /usr/local/bin/git \
    && ln -sf /usr/local/bin/proxy_wrapper.py /usr/local/bin/gh \
    && chmod 711 /usr/bin/git /usr/bin/gh

WORKDIR /workspace
ENV USERNAME=$USERNAME

USER root

ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]

CMD ["/bin/bash"]