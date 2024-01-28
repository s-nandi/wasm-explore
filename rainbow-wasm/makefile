all: bin/prog.js bin/prog.wasm

bin/prog.js bin/prog.wasm: bin/source.wasm bin/libDeps.so 
	emcc -sMAIN_MODULE=1 bin/source.wasm bin/libDeps.so -o bin/prog.js -s STANDALONE_WASM

bin/source.wasm: assets/source.cpp
	emcc -fPIC -c assets/source.cpp -o bin/source.wasm -Ihttp_utilities -Iscene_utilities

bin/libDeps.so: bin/entrypoint.wasm bin/scene_utilities.wasm bin/http_utilities.wasm
	emcc -sSIDE_MODULE bin/entrypoint.wasm bin/scene_utilities.wasm bin/http_utilities.wasm -o bin/libDeps.so

bin/entrypoint.wasm: entrypoint/*
	emcc -fPIC -c entrypoint/*.cpp -o bin/entrypoint.wasm

bin/scene_utilities.wasm: scene_utilities/*
	emcc -fPIC -c scene_utilities/*.cpp -o bin/scene_utilities.wasm

bin/http_utilities.wasm: http_utilities/*
	emcc -fPIC -c http_utilities/*.cpp -o bin/http_utilities.wasm

clean:
	rm -f bin/*