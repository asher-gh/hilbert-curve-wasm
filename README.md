# Hilbert Curve in WebAssembly

Exploring [WebAssembly][wasm] by drawing [Hilbert curve][hc].

[wasm]: https://webassembly.org/
[hc]: https://en.wikipedia.org/wiki/Hilbert_curve

## 🚴 Usage

1. Install [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/) if not installed already.

```sh
cargo install wasm-pack
```

1. Use `wasm-pack` to compile `wasm` and generate JS bindings

```sh
# ./$CARGO_ROOT
wasm-pack build
```

1. Install npm dependencies (from `./www` directory)

```sh
# ./$CARGO_ROOT/www
npm i
```

1. Start the dev server

```sh
# ./$CARGO_ROOT/www
npm run start
```
