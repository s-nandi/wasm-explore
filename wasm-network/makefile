TARGET=wasm32-unknown-unknown

all: bin/app.wasm bindings
	mkdir -p output

bindings: bin/app.wasm
	python3 -m wasmtime.bindgen $< --out-dir hostbindings

bin/app.wasm: app/src/*.rs
	cargo component build --target $(TARGET) --manifest-path="app/Cargo.toml"
	mkdir -p bin
	cp -f app/target/$(TARGET)/debug/app.wasm $@

clean:
	rm -f app/src/bindings.rs
	rm -rf app/target/*
	rm -rf bin
	rm -rf hostbindings
	rm -rf output/*
