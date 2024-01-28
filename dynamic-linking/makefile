# https://stackoverflow.com/a/32168987

TARGET = prog
BIN = bin
CXX = g++
CXXFLAGS = -std=c++20

$(BIN)/$(TARGET): $(BIN)/greeter.so $(BIN)/counter.so $(BIN)/main.o
	$(CXX) $(CXXFLAGS) $^ -L bin -l :greeter.so -l:counter.so -o $@

$(BIN)/main.o: app/main.cpp greeter/include counter/include
	$(CXX) $(CXXFLAGS) -c $< -o $@ -I greeter/include -I counter/include

$(BIN)/greeter.so: $(BIN)/greeter.o
	$(CXX) $(CXXFLAGS) -shared -fPIC -o $@ $<

$(BIN)/counter.so: $(BIN)/counter.o
	$(CXX) $(CXXFLAGS) -shared -fPIC -o $@ $<

$(BIN)/greeter.o: greeter/src/*.cpp
	$(CXX) $(CXXFLAGS) -c -fPIC -o $@ $< -I greeter/include

$(BIN)/counter.o: counter/src/*.cpp
	$(CXX) $(CXXFLAGS) -c -fPIC -o $@ $< -I counter/include

clean:
	rm -f $(BIN)/*