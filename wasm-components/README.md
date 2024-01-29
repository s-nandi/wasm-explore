# Wasm Components

Basically [https://github.com/wasmtime/.../wasi-preview1-component-adapter](https://github.com/bytecodealliance/wasmtime/tree/main/crates/wasi-preview1-component-adapter#using)
along with an understanding of reactor implementation based on this wasmcloud [blog post](https://wasmcloud.com/blog/webassembly-patterns-command-reactor-library) on webassembly patterns.

Cd here via: `cd wasm-components`

Generate wasm modules/components:
```
make # This generates the base .wasm files for `component` and `reactor`, and then component-ifies them using component adapters (see the `Adaptors Source` section below for details)
     # This also uses cargo component to set up `bycargocomponent`, `bycargoreactor`, and `bycargoserver`
```

## Checking component dependencies

After running `make`, run:
```
wasm-tools component wit bin/command-component.wasm  # you should see wasi:cli/run exported
wasm-tools component wit bin/reactor-component.wasm # nothing exported, reactors are leaf nodes
wasm-tools component wit bin/bycargoserver.wasm # you should see wasi:http/types imported and wasi:http/incoming-handler exported, just like wasi:http/proxy
```

## Validating components:
First run `make`, then run:
```
for wasm in bin/*.wasm;
    do  wasm-tools validate $wasm --features component-model;
done
```

Note: if you want to convince yourself that does anything (there's no output), remove the `--features component-model` flag, and you should start seeing errors for the componentified (and cargo component) wasms.

## Merging components:



## Running wasi:cli/command components and modules

Run:
```
# modules:
wasmtime bin/command.wasm

# components:
# apparently the `--wasm component-model` flag is optional now?
# wasmtime seems to autodetect this is a component fine
# meanwhile wasmer throws a validation error on `wasmer bin/command-component.wasm`
# even though it can deal with the module `wasmer bin/command.wasm`, so clearly
# it is getting adapted into a component
wasmtime bin/command-component.wasm
wasmtime bin/bycargocommand.wasm 
```

Note: The reactor components/modules cannot be run via `wasmtime bin/reactor-component.wasm` or `wasmtime bin/reactor.wasm` since `wasmtime run` (which `wasmtime` is an alias for) expects a `run` to be exported, similarly, `wasmtime bin/bycargoreactor.wasm` does not work either

## Serving wasi:http/server components

To serve the server handler built by `bycargoserver`, run:
```
wasmtime serve bin/bycargoserver.wasm
```
If you access `localhost:8080/good` (or almost any other path) you should see a 200-level response code in the browser dev tools `Network` tab
Meanwhile if you access `localhost:8080/bad` , you should get a 500-level response code.

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

## Adaptors Source (needed if spinning up a new repo, already checked into this one)
The adaptors in the `adaptors` folder come from here: https://github.com/bytecodealliance/wasmtime/releases/tag/dev
as described in: https://github.com/bytecodealliance/wasmtime/tree/main/crates/wasi-preview1-component-adapter
The `wasi_snapshot_preview1.wasm` referred to in the `Using` section refers to a reactor adaptor