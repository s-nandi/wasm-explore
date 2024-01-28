# https://stackoverflow.com/a/32168987

TARGET = prog
BIN = bin
CXX = g++
CXXFLAGS = -std=c++20

$(BIN)/$(TARGET): $(BIN)/main.o $(BIN)/lib.a
	$(CXX) $(CXXFLAGS) $^ -o $@

$(BIN)/main.o: app/main.cpp app/lib.hpp greeter/include counter/include
	$(CXX) $(CXXFLAGS) -c $< -o $@ -I greeter/include -I counter/include
     
$(BIN)/lib.a: $(BIN)/lib1.o $(BIN)/lib2.o
	ar rcs $@ $^

$(BIN)/lib1.o: greeter/src/*.cpp
	$(CXX) $(CXXFLAGS) -c -o $@ $< -I greeter/include

$(BIN)/lib2.o: counter/src/*.cpp
	$(CXX) $(CXXFLAGS) -c -o $@ $< -I counter/include

clean:
	rm -f $(BIN)/*