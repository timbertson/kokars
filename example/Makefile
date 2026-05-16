.PHONY: all hello

all: hello

hello:
	cargo build
	koka -e --cclibdir=../target/debug --include=../lib hello.kk -r
