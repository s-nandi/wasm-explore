CXX = emcc
CXXFLAGS = -std=c++20
BIN = bin
INPUTS = app.cpp b.cpp a.cpp

all: $(BIN)/app.wasm $(BIN)/app.html $(BIN)/app.js

$(BIN)/app.wasm: $(INPUTS)
	$(CXX) $(CXXFLAGS) $^ -o $@

$(BIN)/app.html: $(INPUTS)
	$(CXX) $(CXXFLAGS) $^ -o $@

$(BIN)/app.js: $(INPUTS)
	$(CXX) $(CXXFLAGS) $^ -o $@

clean:
	rm -f $(BIN)/*