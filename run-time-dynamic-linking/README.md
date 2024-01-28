# Run-Time Dynamic Linking

First cd into this folder from the project root: `cd run-time-dynamic-linking`

Then run:
```
make
LD_LIBRARY_PATH=bin ./bin/main adder.so 5
```
which should print 7

or:
```
make
LD_LIBRARY_PATH=bin ./bin/main multiplier.so 5
```

To rebuild from scratch, you can optionally first run:
```
make clean
```
