all: bin/reactor-component.wasm bin/command-component.wasm bin/bycargoreactor.wasm bin/bycargocommand.wasm bin/bycargoserver.wasm

bin/reactor-component.wasm: bin/reactor.wasm
	wasm-tools component new $< --adapt adaptors/wasi_snapshot_preview1.reactor.wasm -o $@

bin/reactor.wasm: reactor/reactor.rs
	rustc $< --target wasm32-wasi -o $@

bin/command-component.wasm: bin/command.wasm
	wasm-tools component new $< --adapt adaptors/wasi_snapshot_preview1.command.wasm -o $@

bin/command.wasm: command/command.rs
	rustc $< --target wasm32-wasi -o $@

bin/bycargoreactor.wasm: bycargoreactor/src/*.rs
	cargo component build --target wasm32-unknown-unknown --manifest-path="bycargoreactor/Cargo.toml"
	cp -f bycargoreactor/target/wasm32-unknown-unknown/debug/bycargoreactor.wasm $@

bin/bycargocommand.wasm: bycargocommand/src/*.rs
	cargo component build --manifest-path="bycargocommand/Cargo.toml"
	cp -f bycargocommand/target/wasm32-wasi/debug/bycargocommand.wasm $@

bin/bycargoserver.wasm: bycargoserver/src/*.rs
	cargo component build --manifest-path="bycargoserver/Cargo.toml"
	cp -f bycargoserver/target/wasm32-wasi/debug/bycargoserver.wasm $@

clean:
	rm -f bin/*