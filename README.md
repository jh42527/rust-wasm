# WASI Playground

## Hello World

`cd hello-world`

### Build

Wasi - `cargo wasi build --release`

### Debug 

`cargo wasi run`

### Run

`wasmtime --invoke print_hello target/wasm32-wasi/release/hello_world.wasm --allow-precompiled`