extern crate cbindgen;

use std::env;

fn main() {
	let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

	cbindgen::Builder::new()
	.with_crate(crate_dir)
	.with_no_includes()
	.with_language(cbindgen::Language::C)
	.generate()
	.map_or_else(
		|error| match error {
			e@cbindgen::Error::ParseSyntaxError { .. } => {
				println!("cargo::warning=MESSAGE {:?}", e);
				// Don't fail, the compiler should give a better message
			}
			e => panic!("{:?}", e),
		},
		|bindings| {
			bindings.write_to_file("generated/rust.h");
		},
	);
}
