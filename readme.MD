## Test wasm + bevy

### Running locally

### Native 
```shell
cargo run
```

### Build wasm

```shell
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli

# build
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out --target web ./target/wasm-unknown-unknown/release/test-wasm.wasm
npx serve .
```