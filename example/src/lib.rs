#![no_std]
#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]

extern crate alloc;

use kokars::kklib::*;
use libc_print::std_name::println;
use alloc::boxed::Box;
use alloc::format;

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

#[repr(C)]
pub struct Handle {
	id: i32,
}

// kk_hello__rust_handle has the same layout as KkBoxWrapper<Handle>, but we
// need to create a rust-only alias and use koka's generated C name so that the C compiler is happy.
/// cbindgen:ignore
type kk_hello__rust_handle = KkBoxWrapper<Handle>;

#[unsafe(no_mangle)]
pub extern "C" fn kk_generate_handle(i: i32, ctx: KkContext) -> kk_hello__rust_handle {
	println!("Allocating handle: {}", i);
	KkBoxWrapper::new(KkBox::new(kk_free_handle, Handle { id: i }, ctx))
}

#[unsafe(no_mangle)]
pub extern "C" fn kk_show_handle(h: kk_hello__rust_handle, ctx: KkContext) -> kk_string_t {
	let ptr = h.value.as_ref(ctx);
	let shown = format!("Handle({})", ptr.id);
	kk_string_alloc_dup_valid_utf8(&shown, ctx)
}

impl Drop for Handle {
	fn drop(&mut self) {
		println!("Dropping handle: {}", self.id);
	}
}

// TODO this could be generated with a macro
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kk_free_handle(h: Box<Handle>, _block: *const kk_block_t, _ctx: KkContext) {
	drop(h)
}
