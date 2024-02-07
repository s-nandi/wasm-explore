# Wasm Network

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
