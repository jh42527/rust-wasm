# Image Grayscale API

### Build

`cargo wasi build --release`

### Debug 

`cargo wasi run`

### Run

`wasmedge target/wasm32-wasi/release/image-http-grayscale.wasm`

### Build OCI Container

`docker buildx build --platform wasi/wasm -t rustwasm/image-http-grayscale .`

### Run Docker

`docker run -p=8080:8080 --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm rustwasm/image-http-grayscale .`

### Access Web

http://localhost:8080