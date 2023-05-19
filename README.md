# WASM Playground

## Hello World

`cd hello-world`

### Build

Emscripten (Web) -> `wasm-pack build --target web`
Wasi (External) -> `cargo wasi build --release`

### Run

Debug - cargo wasi run

Web (CLI) - Chrome Index.html
Wasi - wasmtime run wasm/target/wasm32-wasi/release/hello-world.wasm --allow-precompiled
