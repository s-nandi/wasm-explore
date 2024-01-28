all: bin/prog.out

bin/prog.out: bin/source.o bin/libDeps.so 
	g++ bin/source.o -Lbin -lDeps -o bin/prog.out 

bin/source.o: assets/source.cpp
	g++ -c -fPIC assets/source.cpp -o bin/source.o -Ihttp_utilities -Iscene_utilities

bin/libDeps.so: bin/entrypoint.o bin/scene_utilities.o bin/http_utilities.o
	g++ -fPIC -shared bin/entrypoint.o bin/scene_utilities.o bin/http_utilities.o -o bin/libDeps.so

bin/entrypoint.o: entrypoint/*
	g++ -c -fPIC entrypoint/entrypoint.cpp -o bin/entrypoint.o

bin/scene_utilities.o: scene_utilities/*
	g++ -c -fPIC scene_utilities/scene_utilities.cpp -o bin/scene_utilities.o

bin/http_utilities.o: http_utilities/*
	g++ -c -fPIC http_utilities/http_utilities.cpp -o bin/http_utilities.o

clean:
	rm -f bin/*