all: bin/reactor-component.wasm bin/command-component.wasm

bin/reactor-component.wasm: bin/reactor.wasm
	wasm-tools component new $< --adapt adaptors/wasi_snapshot_preview1.reactor.wasm -o $@

bin/reactor.wasm: reactor/reactor.rs
	rustc $< --target wasm32-wasi -o $@

bin/command-component.wasm: bin/command.wasm
	wasm-tools component new $< --adapt adaptors/wasi_snapshot_preview1.command.wasm -o $@

bin/command.wasm: command/command.rs
	rustc $< --target wasm32-wasi -o $@

clean:
	rm -f bin/*