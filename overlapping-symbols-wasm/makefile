CXX = emcc
CXXFLAGS = -std=c++20
BIN = bin
INPUTS = app.cpp $(BIN)/b.so $(BIN)/a.so

all: $(BIN)/app.wasm $(BIN)/app.html $(BIN)/app.js

$(BIN)/app.wasm: $(INPUTS)
	$(CXX) $(CXXFLAGS) $^ -o $@

$(BIN)/app.html: $(INPUTS)
	$(CXX) $(CXXFLAGS) $^ -o $@

$(BIN)/app.js: $(INPUTS)
	$(CXX) $(CXXFLAGS) $^ -o $@

$(BIN)/a.so: a.cpp
	$(CXX) $(CXXFLAGS) -c -fPIC -shared $^ -o $@

$(BIN)/b.so: b.cpp
	$(CXX) $(CXXFLAGS) -c -fPIC -shared $^ -o $@

clean:
	rm -f $(BIN)/*