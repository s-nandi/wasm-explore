# Wasm Filesystem

Build with `make`

The baked in components should be:
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

  export wasi:cli/run@0.2.0;
}
```

which you can check using:
```
wasm-tools component wit bin/app.wasm
```

Run with:
```
wasmtime --dir=bin bin/app.wasm
```

Clean with `make clean`

## Note on filesystem access

Note that the `.rs` file has to specify the full path: `bin/foo.txt`, ie. the file isn't automatically interpreted as a relative path
In fact, trying a relative path inside `main.rs`, specifically `./foo.txt` does not work either, and throws the same error:
```
thread 'main' panicked at src/main.rs:8:46:
could not create file: Custom { kind: Uncategorized, error: "failed to find a pre-opened file descriptor through which \"./foo.txt\" could be opened" }
```