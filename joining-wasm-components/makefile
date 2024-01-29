all: bin/library1.wasm bin/app.wasm bin/composed.wasm

bin/composed.wasm: bin/library1.wasm bin/app.wasm
	wasm-tools compose bin/app.wasm -d bin/library1.wasm -o $@

bin/library1.wasm: library1/*
	cargo component build --target wasm32-unknown-unknown --manifest-path="library1/Cargo.toml"
	cp -f library1/target/wasm32-unknown-unknown/debug/library1.wasm $@

bin/app.wasm: app/*
	cargo component build --target wasm32-wasi --manifest-path="app/Cargo.toml"
	cp -f app/target/wasm32-wasi/debug/app.wasm $@

clean:
	rm -f bin/*