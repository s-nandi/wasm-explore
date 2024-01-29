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

## To convert `source.wasm` to a component:

Download wasi adaptors from: https://github.com/bytecodealliance/wasmtime/releases/tag/dev

Use as follows:
```
wasm-tools component new bin/source.wasm -o bin/source_as_component.wasm --adapt ~/downloads/wasi_snapshot_preview1.reactor.wasm
wasmtime run --wasm component-model bin/source_as_component.wasm
```

