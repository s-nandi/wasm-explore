all: bin/dynamic.wasm bin/dynamic.js bin/oneshot.js bin/static.js

bin/static.js: bin/static_a.wasm bin/static_b.wasm bin/static_app.wasm
	emcc $^ -o $@ -s STANDALONE_WASM

bin/static_a.wasm: a.cpp
	emcc -c $^ -o $@

bin/static_b.wasm: b.cpp
	emcc -c $^ -o $@

bin/static_app.wasm: app.cpp
	emcc -c $^ -o $@

bin/oneshot.js: app.cpp b.cpp a.cpp
	emcc $^ -o $@ -s STANDALONE_WASM

bin/dynamic.wasm bin/dynamic.js: app.cpp bin/shared_a.wasm bin/shared_b.wasm
	emcc $^ -s MAIN_MODULE=1 -o bin/dynamic.js

bin/shared_a.wasm: a.cpp
	emcc -s SIDE_MODULE=1 $^ -o $@

bin/shared_b.wasm: b.cpp
	emcc -s SIDE_MODULE=1 $^ -o $@

clean:
	rm -f bin/*