# Wasm Components

(basically [https://github.com/wasmtime/.../wasi-preview1-component-adapter](https://github.com/bytecodealliance/wasmtime/tree/main/crates/wasi-preview1-component-adapter#using) along with an understanding of reactor implementation based on this wasmcloud [blog post](https://wasmcloud.com/blog/webassembly-patterns-command-reactor-library)) on webassembly patterns.

Cd here via: `cd wasm-components`

Generate wasm modules/components and check component dependencies:
```
make
wasm-tools component wit bin/command-component.wasm  # you should see wasi:cli/run exported
wasm-tools component wit bin/reactor-component.wasm # nothing exported, reactors are leaf nodes
wasm-tools component wit bin/bycargoserver.wasm # you should see wasi:http/types imported and wasi:http/incoming-handler exported, just like wasi:http/proxy
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

To serve the server handler built by `bycargoserver`, run:
```
wasmtime serve bin/bycargoserver.wasm
```
If you access `localhost:8080/good` you should see a 200-level response code in the browser dev tools `Network` tab
Meanwhile if you access `localhost:8080/bad` , you should get a 500-level response code.

Note: The reactor-component cannot be run via `wasmtime bin/reactor-component.wasm` since it expects a `run` to be exported, similarly, `wasmtime bin/bycargoreactor.wasm` does not work either

## Setting up local WASI package dependencies (needed if spinning up a new repo, already checked into this one):

According to this zulip [comment](https://bytecodealliance.zulipchat.com/#narrow/stream/407292-cargo-component/topic/Moving.20to.20cargo-component.20from.20wit-bindgen/near/413040499), the preview registry wouldn't work, so this is the process for getting wit dependencies locally:

This repo is very helpful as well: [wasi-http-test](https://github.com/landonxjames/wasi-http-test/tree/main)

1. Download the `wit` folder from the [wasi-http](https://github.com/WebAssembly/wasi-http) repo, and place it in `./wit/deps`
1. Restructure `./wit/deps` so that `wasi-http`'s dependencies are pulled up into `./wit/deps` (instead of `./wit/deps/deps`) and move the remaining `wasi-http` .wit's into their own `./wit/deps/http` folder
1. Run:
```
cargo component add --target --path wit/deps/cli wasi:cli
cargo component add --target --path wit/deps/clocks wasi:clocks
cargo component add --target --path wit/deps/filesystem wasi:filesystem
cargo component add --target --path wit/deps/http wasi:http
cargo component add --target --path wit/deps/io wasi:io
cargo component add --target --path wit/deps/random wasi:random
cargo component add --target --path wit/deps/sockets wasi:sockets
```
1. Add `path = "wit/world.wit"` to the `[package.metadata.component.target]` section of `Cargo.toml`

## Adaptors Source
The adaptors in the `adaptors` folder come from here: https://github.com/bytecodealliance/wasmtime/releases/tag/dev
as described in: https://github.com/bytecodealliance/wasmtime/tree/main/crates/wasi-preview1-component-adapter
The `wasi_snapshot_preview1.wasm` referred to in the `Using` section refers to a reactor adaptor