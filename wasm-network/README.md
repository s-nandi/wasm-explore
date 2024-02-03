# Wasm Network

Build with `make`

This first builds Wasi-Virt locally, we need a custom build because the repo currently exports `@0.2.0-rc-2023-10-18` versions of the common WASI interfaces (ex. `wasi:io`, `wasi:http`, etc), while the other componentizers (ex. `componentize-py`) depend on the stable (?) `0.2.0` versions of those interfaces -- this prevents us from composing the output `virt.wasm` with `component.wasm` (ie. `pyapp.wasm`)

Clean with `make clean`
