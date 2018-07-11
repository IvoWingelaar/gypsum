# Gypsum

A Rust library for drawing ASCII-style roguelike graphics targetting both native (GL) and wasm (WebGL) platforms.

# Running

## Native

```
rustup default nightly
cargo run --release --example screen_quad
```

## Wasm

```
rustup default nightly
rustup target install wasm32-unknown-unknown
cargo install cargo-web
cargo web start --release --example screen_quad
```

# License

All work in this repo is licensed under the MIT license.
See `LICENSE` for more information.
Partly based on some work of [doryen-rs](https://github.com/jice-nospam/doryen-rs).
