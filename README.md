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