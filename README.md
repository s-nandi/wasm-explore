# wasm-explore
Toy project for exploring WASM / WASI

## Static linking (C++)

An example of static linking two libraries with an application's source code using makefiles and g++.

Details at: [Static Linking](./static-linking/README.md)

## Dynamic linking (C++)

An example of dynamically linking two libraries with an application's source code using makefiles and g++.

Details at: [Dynamic Linking](./dynamic-linking/README.md)

## Run-time Dynamic Linking (C++)

An example of dynamically calling a function in either of two libraries by using a CLI argument, using makefiles and g++.

Details at [Runtime Dynamic Linking](./run-time-dynamic-linking/README.md)

## WASM Compilation with Overlapping Symbols (Emscripten, C++, Node)

An example of compiling C++ files into wasm when they share a common function name (albeit namespaced), using makefiles, emscripten, g++, and node (to run the compiled output).

Details at: [Overlapping Symbols WASM](./overlapping-symbols-wasm/README.md)

## Emscripten into Wasmer (Emscriptem, C++, Wasmer)

An example of compiling C++ files into a standalone .wasm via Emscripten and then running that via wasmer

Details at [Emscripten into Wasmer](./emscripten-into-wasmer/README.md)

## Linking WASM (Emscripten, C++, Wasmer)

An example of dynamically linking (and statically linking) pre-built wasm modules together, along with timing results comparing one-shot builds (compile and link in one step), static linking, and dynamic linking. Notably, Wasmer cannot consume the dynamic library output from Emscripten, but the `.js` output from Emscripten can be run by node.

Details at [Linking WASM](./linking-wasm/README.md)

## Rainbow

Test dynamic linking with a 1) user provided source file 2) predetermined entrypoint 3/4) multiple libraries with complex (ie. heap stored) variables.

Details at [Rainbow](./rainbow/README.md)

## Rainbow WASM (Emscriptem)

Test dynamic linking with a 1) user provided source file 2) predetermined entrypoint 3/4) multiple libraries with complex (ie. heap stored) variables via emscripten.

Details at [Rainbow WASM](./rainbow-wasm/README.md)

## Opaque Rainbow (Wasmtime, C++, Emscripten, Rust, Cargo Component)

Test linking multiple languages in one combined wasm (component or module)

Details at [Opaque Rainbow](./opaque-rainbow/README.md)

## Wasm Components (Wasmtime, Rust, Cargo Component)

Test different ways of adapting plain wasm modules into components

Details at [WASM Components](./wasm-components/README.md)

## Joining Wasm Components (Wasmtime, Rust, Cargo Component, Componentize Py)

Test different ways of joining wasm components

Details at [Joining WASM Components](./joining-wasm-components/README.md)

## Wasm Filesystem (Wasmtime, Cargo Component)

Test how accessing the file system works

Details at [Wasm Filesystem](./wasm-filesystem/README.md)

## Wasm Network (Wasmtime, Cargo Component)

Test how making network calls works

Showcases how to make a python host to provide network utilities since WASI-Virt currently uses a different (older) version of WASI components compared to what `cargo component` generates with. The port is non-trivial since various functions/types have moved around in `WASI` between the versions.

Details at [Wasm Network](./wasm-network/README.md)