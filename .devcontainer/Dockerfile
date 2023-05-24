FROM mcr.microsoft.com/devcontainers/rust

WORKDIR /opt/wasmtime

RUN curl https://wasmtime.dev/install.sh -sSf | bash

ENV PATH "$PATH:/root/.wasmtime/bin/;" ls

WORKDIR /opt

RUN git clone https://github.com/emscripten-core/emsdk.git && \
    cd emsdk && \
    ./emsdk install latest && \
    ./emsdk activate latest

RUN echo 'source "/opt/emsdk/emsdk_env.sh"' >> $HOME/.bash_profile

RUN rustup target add wasm32-wasi && rustup target add wasm32-unknown-unknown && rustup target add wasm32-unknown-emscripten

RUN cargo install cargo-wasi && cargo install wasm-pack