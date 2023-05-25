FROM mcr.microsoft.com/devcontainers/rust

# WASMEDGE
WORKDIR /opt/wasmedge

RUN curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash

RUN echo 'source "$HOME/.wasmedge/env"' >> $HOME/.bash_profile

# WASMTIME
WORKDIR /opt/wasmtime

RUN curl https://wasmtime.dev/install.sh -sSf | bash

ENV PATH "$PATH:/root/.wasmtime/bin/;" ls

# RUST CARGO BUILD TOOLS
RUN rustup target add wasm32-wasi && rustup target add wasm32-unknown-unknown

RUN cargo install cargo-wasi && cargo install wasm-pack