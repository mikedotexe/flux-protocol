BASEDIR=$(dirname "$0")
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp ./target/wasm32-unknown-unknown/release/flux_protocol.wasm ./res/flux_protocol.wasm
rm -rf target/wasm32-unknown-unknown