# Joining Wasm Components

The wasm-components dir is a pre-requisite for this (conceptually, not with respect to code)

Inspired by the example of writing a custom library-style component and consuming it in a command-style component, in the webassembly component model introduction's section on [Running Components](https://component-model.bytecodealliance.org/creating-and-consuming/running.html#running-components-with-custom-exports). The [Composing Components](https://component-model.bytecodealliance.org/creating-and-consuming/composing.html) section of the same documentation is also helpful to see how multiple dependencies could be chained together (if you don't want to use [wasmbuilder.app](https://wasmbuilder.app/)).

## Pre-setup (if making for scratch, already checked in here)

Run:
```
cargo component new library1 --lib
cargo component new app --command
```
to make each component

Then make a `app/wit/world.wit` file with the following contents:
```
package component:app;

world example {
    import component:library1/greeter;
}
```

And change the contents of `library1/wit/world.wit` to:
```
package component:library1;

interface greeter {
    hello-world: func() -> string;
}

/// An example world for the component to target.
world example {
    export greeter;
}
```

Then link the dependence on `library1` while in `app` by running:
```
cargo component add --path ../library1/wit --target component:library1
```

You'll also want to replace the contents of `app/src/main.rs` with this:
```
mod bindings;

use crate::bindings::component::library1::greeter::hello_world;

fn main() {
    println!("In main");
    println!("{}", hello_world());
}
```
so the app actually uses the library1 exported `hello-world`` function.

## Make each component separately

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
but with an additional `import component:library1/greeter` -- this comes from the `app/wit/world.wit`, which `cargo component` automatically picks up on (the components of `app/wit/world.wit` are described in the Pre-Setup section)

Note: Since we don't have public registries for wasm components yet, `app/Cargo.toml` has to explicitly point to a local copy of `component:library1`, via a relative path. The necessary `cargo component add ...` command required is documented above in the Pre-Setup section

### Trying to run incomplete apps
If you try to run the app component without satisfying its missing `library1/greeter` dependency:
```
wasmtime bin/app.wasm 
```

You'll see an error because of the unsatisfied import:
```
Error: failed to run main module `bin/app.wasm`

Caused by:
    0: import `component:library1/greeter` has the wrong type
    1: instance export `hello-world` has the wrong type
    2: expected func found nothing
```

## Joining Components

After completing the previous section, you can run (which is also run by make):
```
wasm-tools compose bin/app.wasm -d bin/library1.wasm -o bin/composed.wasm
```

Now if you check the dependencies of this new composed wasm:
```
wasm-tools component wit bin/composed.wasm
```
you should see that it's an exactly match for [wasi:cli/command](https://github.com/WebAssembly/wasi-cli/blob/main/wit/command.wit) unlike earlier, which lets us run it via wasmtime

Try this by running:
```
wasmtime bin/composed.wasm
```

Not only does this work, you should also see:
```
In main
Hello, World!
```
which shows the `hello-world` function exported by `library1` getting used in `app`!