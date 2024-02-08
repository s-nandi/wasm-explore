# Wasm Network - Python

Build with `make`

Then run the host python program with:
```
python3 -m hostpy.host
```

Clean with `make clean`

## Details

The built `bin/app.wasm` requires a `network-provider` and `filesystem-provider` to function, which means we need to write a custom host (the built in adapters for `wasm-tools component new --adapt <adaptor>`, in `adaptors`, only work for standard `command` and `reactor` applications, see [`wit-components`](https://crates.io/crates/wit-component) for details).

The `hostpy/host.py` file showcases a custom host written in python. It isn't as secure as composing `bin/app.wasm` with another `wasm` module, but does allow us to inject arbitrary provider functions written in python, which is especially useful for `fetch` requests.

You can also check out what a self contained host looks like by running [hostpy/self_contained_host_example.py](./hostpy/self_contained_host_example.py):
```
python3 -m hostpy.self_contained_host_example
```
 (the `Func` and `Instance` function calls in `self_contained_host_example.py` are taken care by the `wasmtime.bindgen` step in the [makefile](./makefile))

# Wasm Network - Rust Self Contained

The rust example is self-contained, go into `hostrs`

First let's check what the wit's look like for each wasm:

hello world export, wasi import:
```
wasm-tools component wit hello_world_export_wasi_imports.wasm 
```
should show:
```
world root {
  import wasi:cli/environment@0.2.0;
  import wasi:cli/exit@0.2.0;
  import wasi:io/error@0.2.0;
  import wasi:io/streams@0.2.0;
  import wasi:cli/stdin@0.2.0;
  import wasi:cli/stdout@0.2.0;
  import wasi:cli/stderr@0.2.0;
  import wasi:clocks/wall-clock@0.2.0;
  import wasi:filesystem/types@0.2.0;
  import wasi:filesystem/preopens@0.2.0;

  export hello-world: func() -> string;
}
```

hello world export, no import:
```
wasm-tools component wit hello_world_export.wasm 
```
should show:
```
package root:component;

world root {
  export hello-world: func() -> string;
}
```

command export, wasi import
```
wasm-tools component wit command_export_wasi_imports.wasm 
```
should show:
```
package root:component;

world root {
  import wasi:cli/environment@0.2.0;
  import wasi:cli/exit@0.2.0;
  import wasi:io/error@0.2.0;
  import wasi:io/streams@0.2.0;
  import wasi:cli/stdin@0.2.0;
  import wasi:cli/stdout@0.2.0;
  import wasi:cli/stderr@0.2.0;
  import wasi:clocks/wall-clock@0.2.0;
  import wasi:filesystem/types@0.2.0;
  import wasi:filesystem/preopens@0.2.0;

  export wasi:cli/run@0.2.0;
}
```

Build with:
```
make
```

(you can clean with `make clean`)

Now run:

```
cargo run --bin self_contained
```

You should see: 
```
Starting

Run run_custom_function = true use_file_with_wasi_imports = true
Loading hello_world_export_wasi_imports.wasm
Done loading hello_world_export_wasi_imports.wasm
# requirements = 2
Result: ("Hello, World!",)

Run run_custom_function = true use_file_with_wasi_imports = false
Loading hello_world_export.wasm
Done loading hello_world_export.wasm
# requirements = 1
Result: ("Hello, World!",)

Run run_custom_function = false use_file_with_wasi_imports = true
Loading command_export_wasi_imports.wasm
Done loading command_export_wasi_imports.wasm
# requirements = 2
Hello, world!
Result: Ok(Ok(()))
```

So both hello world exporting functions return 'hello world' but don't print anything
while the command doesn't have a return, but does print 'hello world'

Also interestingly, the wasm without wasi imports (`hello_world_export.wasm`) has 1 module requirement, while the others require 2 (I'm not sure why it's 1 vs 2, instead of 0 vs 5 -- where 5 is the number of wasi:X packages)

# Wasm Network - Rust using Component Client

First cd into `hostrs`

Build with:
```
make
```
This first builds a `hello_world_export_wasi_and_myimport_imports.wasm` from `app2/src/lib.rs` that just gets placed directly in `hostrs`

You can check its imports with:
```
wasm-tools component wit hello_world_export_wasi_and_myimport_imports.wasm
```
which should show:
```
package root:component;

world root {
  import component:dep/dep;

  export hello-world: func() -> string;
}
```

Running `make` then builds `src_using_app2/main.rs`, which provides a host implementation for the `component:dep/dep` import required by the root component.

if you run this, with:
```
cargo run --bin using_app2
```
you should see:
```
Starting

Loading hello_world_export_wasi_and_myimport_imports.wasm
Done loading hello_world_export_wasi_and_myimport_imports.wasm
# requirements = 2
Result: ("Hello world with import (imported string into interface)",)
```
where the `imported string into interface` part of the result comes from `app2`


Clean with:
```
make clean
```