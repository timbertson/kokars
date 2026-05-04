extern crate cbindgen;

use std::env;
use std::process::Command;
use std::path::Path;

fn main() {
	let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	
	let generate_kk = "bootstrap/generate.kk";
	println!("cargo::warning=Bootstrapping koka types...");
	println!("cargo::rerun-if-changed={}", generate_kk);
	let bootstrap = Command::new("koka")
		.arg("--verbose=0")
		.arg("-e")
		.arg(generate_kk)
		.output()
		.expect("command failed");
	
	if !bootstrap.status.success() {
		let stderr = String::from_utf8(bootstrap.stderr).unwrap();
		for line in stderr.lines() {
			println!("cargo::warning={}", line);
		}

		let stdout = String::from_utf8(bootstrap.stdout).unwrap();
		for line in stdout.lines() {
			println!("cargo::warning={}", line);
		}
		panic!("bootstrap failed");
	}
	let dest_file = Path::new(crate_dir.as_str()).join("src").join("kklib/generated/mod.rs");
	std::fs::create_dir_all(dest_file.parent().unwrap()).unwrap();
	std::fs::write(dest_file, bootstrap.stdout).unwrap();

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
			bindings.write_to_file("generated/kokars.h");
		},
	);
}
