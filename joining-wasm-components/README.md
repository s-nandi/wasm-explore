# Joining Wasm Components

The wasm-components dir is a pre-requisite for this (conceptually, not with respect to code)

Inspired by the example of writing a custom library-style component and consuming it in a command-style component, in the webassembly component model introduction's section on [Running Components](https://component-model.bytecodealliance.org/creating-and-consuming/running.html#running-components-with-custom-exports). The [Composing Components](https://component-model.bytecodealliance.org/creating-and-consuming/composing.html) section of the same documentation is also helpful to see how multiple dependencies could be chained together (if you don't want to use [wasmbuilder.app](https://wasmbuilder.app/)).

## Pre-setup (if making for scratch, already checked in here)

Run:
```
cargo component new library1 --lib
cargo component new app --command
```

## Running

cd into this directory, `cd joining-wasm-components`

Then run:
```
make
```

If you inspect the library1 component:
```
wasm-tools component wit bin/library1.wasm
```
you should see `hello-world: funct() -> string` being the only export

meanwhile if you inspect app:
```
wasm-tools component wit bin/app.wasm
```
you'll see something that's almost an exact match for [wasi:cli/command](https://github.com/WebAssembly/wasi-cli/blob/main/wit/command.wit)
but with an additional `import hello-world: funct() -> string` -- this comes from the `app/wit/world.wit`, which `cargo component` automatically picks up on

For reference, these are the contents of `app/wit/world.wit`:
```
package component:app;

world example {
    import hello-world: func() -> string;
}
```