# 5:31 stop

# TODO: move generation into build.rs
GENERATED_RUST_TYPEDEFS=src/kokars/kklib.rs

.PHONY: all lib cargo $(GENERATED_RUST_TYPEDEFS)

all: cargo

$(GENERATED_RUST_TYPEDEFS):
	koka -e bootstrap/generate.kk > $@

cargo:
	cargo build
