# Opaque Rainbow

Cd into this directory: `cd opaque-rainbow`

Run:
```
make
wasmtime run bin/source.wasm
wasmtime run --wasm component-model bin/rust.wasm
```

TODO: Bundle the rust and cpp code into one component

Clean:
```
make clean
```