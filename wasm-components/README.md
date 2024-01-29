# Wasm Components

(basically [https://github.com/wasmtime/.../wasi-preview1-component-adapter](https://github.com/bytecodealliance/wasmtime/tree/main/crates/wasi-preview1-component-adapter#using) along with an understanding of reactor implementation based on this wasmcloud [blog post](https://wasmcloud.com/blog/webassembly-patterns-command-reactor-library)) on webassembly patterns.

Cd here via: `cd wasm-components`

Generate wasm modules/components and check component dependencies:
```
make
wasm-tools component wit bin/command-component.wasm  # you should see wasi:cli/run exported
wasm-tools component wit bin/reactor-component.wasm # nothing exported, reactors are leaf nodes
```

Run:
```
# apparently the `--wasm component-model` flag is optional now?
# wasmtime seems to autodetect this is a component fine
# meanwhile wasmer throws a validation error on `wasmer bin/command-component.wasm`
# even though it can deal with the module `wasmer bin/command.wasm`, so clearly
# it is getting adapted into a component
wasmtime bin/command-component.wasm
wasmtime bin/bycargocommand.wasm
```

Note: The reactor-component cannot be run via `wasmtime bin/reactor-component.wasm` since it expects a `run` to be exported, similarly, `wasmtime bin/bycargoreactor.wasm` does not work either

## Adaptors Source
The adaptors in the `adaptors` folder come from here: https://github.com/bytecodealliance/wasmtime/releases/tag/dev
as described in: https://github.com/bytecodealliance/wasmtime/tree/main/crates/wasi-preview1-component-adapter
The `wasi_snapshot_preview1.wasm` referred to in the `Using` section refers to a reactor adaptor