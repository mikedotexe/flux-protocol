<div align="center">

  <h1><code>flux-protocol</code></h1>

  <p>
    <strong>Open market protocol, build on NEAR.</strong>
  </p>

</div>

## Pre-requisites
To develop Rust contracts you would need to:
* Install [Rustup](https://rustup.rs/):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
* Add wasm target to your toolchain:
```bash
rustup target add wasm32-unknown-unknown
```
* Clone the Flux monorepo 
```
git clone https://github.com/jasperdg/flux-protocol.git
```

## Running tests
Navigate to the protocol directory

```
cd protocol
```

Run the test

```
cargo test --package flux-protocol
```

## Side notes
To test using an existing version of the protocol please checkout [deployment](https://github.com/jasperdg/flux-protocol/tree/master/deployment)