# https://stackoverflow.com/a/32168987

TARGET = prog
BIN = bin
CXX = g++
CXXFLAGS = -std=c++20 -Wall -g

all: $(BIN)/adder.so $(BIN)/multiplier.so $(BIN)/main

$(BIN)/main: app/main.cpp adder/include multiplier/include
	$(CXX) $(CXXFLAGS) $< -o $@ -I adder/include -I multiplier/include

$(BIN)/adder.so: adder/src/*.cpp
	$(CXX) $(CXXFLAGS) -shared -fPIC -o $@ $< -I adder/include

$(BIN)/multiplier.so: multiplier/src/*.cpp
	$(CXX) $(CXXFLAGS) -shared -fPIC -o $@ $< -I multiplier/include

clean:
	rm -f $(BIN)/*