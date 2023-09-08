## Prerequisites

First, make sure you have Python 3.10 or later and `pip` installed.  Then, install `componentize-py`:

```
pip install componentize-py
```

## Building

```
componentize-py -d wit -w hello-name componentize app -o app.wasm
```

## Sanity check

You'll need to install Rust first.  Then:

```
cargo run --release --manifest-path runner/Cargo.toml -- app.wasm
```
