# Dynamic Linking and Static Linking in WASM
First cd into this folder from the project root: `cd linking-wasm`

Run
```
make
cd bin
node app.js
```

Note: Dynamic linking does not play well with WASI (ie. standalone exceution, such as via Wasmer) currently
See this github [thread](https://github.com/WebAssembly/wasi-sdk/issues/332#issuecomment-1711245468) for a related discussion

This incompatibility is why we build a `.js` file instead, and run it via node -- emscripten handles the dynamic loading functionality in javascript land.

Extern "C" linkage wouldn't solve the issue in general because that prevents us from passing C++ only types (ex. vector, heap allocating classes in general) across modules. Not sure if this helps if we only pass basic types back and forth.

# Timing comparisons (one shot build vs static link vs dynamic link)

To prove that this speeds up the final build (assuming the individual `a.wasm` and `b.wasm` dynamic libraries are pre-built), compare the following:

(note: the `.js` artifact is only required when dynamic linking since wasmer is happy to run the `.wasm` artifact otherwise, but to get a fair comparison, we just output the `.js` artifact in all cases)

## One shot

Run
```
time emcc app.cpp b.cpp a.cpp -o bin/oneshot.js -s STANDALONE_WASM
```
For reference, this takes about 1.7 - 1.8 seconds on my system currently

Note: This doesn't depend on dynamic linking, so this can actually be run on wasmer via: `wasmer bin/oneshot.wasm`


## Static Linking

First pre-compile each library and the application via:
```
emcc -c a.cpp -o bin/static_a.wasm
emcc -c b.cpp -o bin/static_b.wasm
emcc -c app.cpp -o bin/static_app.wasm
```

Then link them together (and time it) via:
```
time emcc bin/static_a.wasm bin/static_b.wasm bin/static_app.wasm -o bin/static.js -s STANDALONE_WASM
```
This runs in about 0.27 - 0.29 seconds for me currently

By the way, using `app.cpp` in the final step instead of the pre-compiled `bin/static_app.wasm`, ie. via:
```
time emcc bin/static_a.wasm bin/static_b.wasm app.cpp -o bin/static.js -s STANDALONE_WASM
```
is slower (as expected, since app isn't compiled ahead of time), but for reference, this takes about 0.75 - 0.8 seconds for me currently

Note: This doesn't depend on dynamic linking, so again, this can be run on wasmer via: `wasmer bin/static.js`

## Dynamic Linking

First pre-compile each library as side modules via:
```
emcc -sSIDE_MODULE a.cpp -o bin/shared_a.wasm
emcc -sSIDE_MODULE b.cpp -o bin/shared_b.wasm
```

Then dynamically link with the application via:
```
time emcc app.cpp -sMAIN_MODULE bin/shared_a.wasm bin/shared_b.wasm -o bin/dynamic.js -s STANDALONE_WASM
```
This runs in 1.2 - 1.3 seonds for me

Note: This cannot be run via wasmer, the `-s STANDALONE_WASM` flag isn't actually applicable (see above), so `wasmer bin/dynamic.wasm` fails