TARGET=wasm32-unknown-unknown

all: src

src: hello_world_export_wasi_and_myimport_imports.wasm
	cargo build

hello_world_export_wasi_and_myimport_imports.wasm:
	cd app2 && cargo component build --target $(TARGET) && cp target/$(TARGET)/debug/app2.wasm ../$@

clean:
	rm -rf app2/target
	rm -rf target
	rm hello_world_export_wasi_and_myimport_imports.wasm