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