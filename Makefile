RUSTC ?= rustc

dummy1 := $(shell mkdir bin 2> /dev/null)

all:
	$(RUSTC) -o bin/snake -L lib/ src/main.rs

clean:
	rm -rf bin