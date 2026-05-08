use kokars::kklib::*;

#[unsafe(no_mangle)]
pub extern "C" fn kk_hello_rs(_c: KkContext) {
	println!("Hello from Rust!");
}

#[unsafe(no_mangle)]
pub extern "C" fn kk_hello_rs_str(kk_str: kk_string_t, ctx: KkContext) {
	let str: &str = kk_string_as_str(&kk_str, ctx);
	// drop(kk_str); // wouldn't compile!
	println!("koka-string:[[{}]]", str);
}

#[unsafe(no_mangle)]
pub extern "C" fn kk_kkrs_callback1(kk_str: kk_string_t, cb: kk_function_t, ctx: KkContext) -> kk_integer_t {
	println!("calling function {:?} with stringval {}", cb, kk_string_as_str(&kk_str, ctx));
	unsafe {
		kk_function_call_1::<kk_string_t, kk_integer_t>(cb, kk_str, ctx)
	}
}
