FROM --platform=$BUILDPLATFORM rust AS buildbase
WORKDIR /src
RUN <<EOT bash
    set -ex
    apt-get update
    apt-get install -y \
        git \
        clang
    rustup target add wasm32-wasi
EOT

FROM buildbase AS build
COPY Cargo.toml .
COPY src ./src 
# Build the Wasm binary
RUN cargo build --target wasm32-wasi --release

FROM scratch
COPY --from=build /src/target/wasm32-wasi/release/fibonacci.wasm /fibonacci.wasm
ENTRYPOINT [ "/fibonacci.wasm" ]