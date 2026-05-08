.PHONY: all hello

all: hello

hello:
	cargo build
	koka -e --cclibdir=../target/debug --include=../lib --optimize=2 hello.kk


