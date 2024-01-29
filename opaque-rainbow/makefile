all: bin/source.wasm bin/rust.wasm

bin/source.wasm: assets/source.cpp bin/entrypoint.wasm bin/scene_utilities.wasm bin/http_utilities.wasm
	emcc -fPIC $^ -o $@ -Ihttp_utilities -Iscene_utilities -s STANDALONE_WASM

bin/entrypoint.wasm: entrypoint/*
	emcc -fPIC -c entrypoint/*.cpp -o $@

bin/scene_utilities.wasm: scene_utilities/*
	emcc -fPIC -c scene_utilities/*.cpp -o $@

bin/http_utilities.wasm: http_utilities/*
	emcc -fPIC -c http_utilities/*.cpp -o $@

bin/rust.wasm: rust/src/*.rs
	cd rust; cargo component build --target wasm32-wasi --release; cp -f target/wasm32-wasi/release/rust.wasm ../bin

clean:
	rm -f bin/*