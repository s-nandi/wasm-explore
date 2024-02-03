all: bin/wasi-virt bin/virt.wasm # bin/app.wasm bin/pyapp.wasm

bin/app.wasm: app/src/*.rs
	cargo component build --manifest-path="app/Cargo.toml"
	mkdir -p bin
	cp -f app/target/wasm32-unknown-unknown/debug/app.wasm $@

bin/virt.wasm: bin/wasi-virt
	./bin/wasi-virt --out bin/virt.wasm

bin/wasi-virt: WASI-Virt/target/release/wasi-virt
	cp WASI-Virt/target/release/wasi-virt bin

WASI-Virt/target/release/wasi-virt: WASI-Virt/target/wasm32-wasi/release/virtual_adapter.wasm
	cd WASI-Virt && cargo build --release

WASI-Virt/target/wasm32-wasi/release/virtual_adapter.wasm:
	cd WASI-Virt && ./build-adapter.sh

bin/pyapp.wasm: pyapp/src.py
	cd pyapp && componentize-py -d wit -w bindings bindings .
	mkdir -p bin
	cd pyapp && componentize-py -d wit -w bindings componentize src -o ../bin/pyapp.wasm

clean:
	rm -f app/src/bindings.rs
	rm -rf app/target/*
	rm -f bin/*.wasm
	rm -rf pyapp/bindings/*