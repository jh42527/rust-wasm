# WASM Playground

## Hello World

`cd rust/hello-world`

### Build

cargo wasi build --release

### Run

Debug - cargo wasi run

Release - wasmtime run /target/wasm32-wasi/release/hello-world.wasm --allow-precompiled
