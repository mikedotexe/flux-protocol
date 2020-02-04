cargo +nightly build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/flux_protocol.wasm ./res/
wasm-opt -Oz --output ./res/flux_protocol.wasm ./res/flux_protocol.wasm
wasm-gc ./res/flux_protocol.wasm
rm -rf target