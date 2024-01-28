all: bin/app.wasm bin/app.js

bin/app.wasm bin/app.js: app.cpp bin/a.wasm bin/b.wasm
	emcc app.cpp bin/a.wasm bin/b.wasm -s MAIN_MODULE=1 -o bin/app.js

bin/a.wasm: a.cpp
	emcc -s SIDE_MODULE=1 a.cpp -o bin/a.wasm

bin/b.wasm: b.cpp
	emcc -s SIDE_MODULE=1 b.cpp -o bin/b.wasm

clean:
	rm -f bin/*