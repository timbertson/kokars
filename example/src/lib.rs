use kokars::kklib::*;

#[unsafe(no_mangle)]
pub extern "C" fn kk_hello_rs(_c: &kk_context_t) {
	println!("Hello from Rust!");
}

#[unsafe(no_mangle)]
pub extern "C" fn kk_hello_rs_str(kk_str: kk_string_t, ctx: &kk_context_t) {
	let str: &str = kk_string_as_str(&kk_str, &ctx);
	// drop(kk_str); // wouldn't compile!
	println!("koka-string:[[{}]]", str);
}
