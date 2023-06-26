```
rustup target add wasm32-wasi # install wasm as target
cargo build --target wasm32-wasi # build targeting wasm
cp ../target/wasm32-wasi/debug/wasm_example.wasm ./inc.wasm
python3 -m http.server
```