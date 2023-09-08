## Prerequisites

First, make sure you have Python 3.10 or later and `pip` installed.  Then, install `componentize-py`:

```
pip install componentize-py
```

## Building

```
componentize-py -d wit -w hello componentize app -o app.wasm
```

## Sanity check

You'll need Wasmtime 13 with Component Model support enabled.  First install Rust, then:

```
cargo install --locked --git https://github.com/bytecodealliance/wasmtime --branch release-13.0.0 --features component-model wasmtime-cli
```

Then, run the component using:

```
wasmtime --wasm-features component-model app.wasm
```
