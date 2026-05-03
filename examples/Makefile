.PHONY: all hello rs/examples-rust.h

all: hello

# produced by build.rs; also produces the library file
rs/examples-rust.h:
	cd rs && cargo build

hello: rs/examples-rust.h
	koka -e --cclibdir=$(pwd)rs/target/debug hello.kk


