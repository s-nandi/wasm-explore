all: bin/app.wasm

bin/app.wasm: app/src/*.rs
	cargo component build --manifest-path="app/Cargo.toml"
	mkdir -p bin
	cp -f app/target/wasm32-wasi/debug/app.wasm $@

clean:
	rm -f app/src/bindings.rs
	rm -rf app/target/*
	rm -rf bin/* 