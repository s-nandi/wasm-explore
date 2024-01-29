all: bin/library1.wasm bin/app.wasm bin/composed.wasm bin/library2.wasm

bin/composed.wasm: bin/library1.wasm bin/app.wasm bin/library2.wasm
	wasm-tools compose bin/app.wasm -d bin/library1.wasm -d bin/library2.wasm -o $@

bin/library1.wasm: library1/src/*.rs library1/wit/*.wit
	cargo component build --target wasm32-unknown-unknown --manifest-path="library1/Cargo.toml"
	cp -f library1/target/wasm32-unknown-unknown/debug/library1.wasm $@

bin/app.wasm: app/src/*.rs app/wit/*.wit
	cargo component build --target wasm32-wasi --manifest-path="app/Cargo.toml"
	cp -f app/target/wasm32-wasi/debug/app.wasm $@

bin/library2.wasm: library2/src/*.py library2/generatorworld/types.py
	cd library2; componentize-py -d wit/python.wit -w generatorworld componentize src.main -o ../$@

bindings library2/generatorworld/types.py: library2/wit/*.wit
	rm -rf library2/generatorworld
	componentize-py -d library2/wit/python.wit -w generatorworld bindings library2

clean_all: clean clean_bindings

clean:
	rm -f bin/*

clean_bindings:
	rm -rf library2/generatorworld
	rm app/src/bindings.rs
	rm library1/src/bindings.rs