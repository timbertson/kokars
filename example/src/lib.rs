#![no_std]

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

// unsafe extern "C" {
// 	pub fn kk_hello__rust_handle_drop(h: Uncounted<kk_hello__rust_handle>, ctx: *const kk_context_t);
// }

// The C type matching what the koka compiler will generate from hello.kk `rust-handle` structure.
// Manage this carefully, it cannot currently be derived from the generated koka code
/// cbindgen:ignore
#[repr(C)]
pub struct kk_hello__rust_handle {
	internal: kk_box_t,
}
// impl Drop for kk_hello__rust_handle {
// 	fn drop(&mut self) {
// 		println!("dropping kk_hello_rust_handle");
// 		unsafe {
// 			kk_hello__rust_handle_drop(Uncounted::unsafe_from_ref(self), kk_get_context());
// 		}
// 	}
// }

#[unsafe(no_mangle)]
pub extern "C" fn kk_generate_handle(i: i32, ctx: KkContext) -> kk_box_t {
	println!("Allocating handle: {}", i);
	let h = Box::new(Handle { id: i });
	let h_ptr: *const Handle = &*h;
	println!("box ptr = {:?}", h_ptr);
	kk_raw_box(kk_free_handle, h, ctx)
}

#[unsafe(no_mangle)]
// pub extern "C" fn kk_show_handle(h: kk_hello__rust_handle, ctx: KkContext) -> kk_string_t {
// 	let ptr = unsafe { kk_box_to_ptr::<Handle>(&h.internal, ctx) };
pub extern "C" fn kk_show_handle(h: kk_box_t, ctx: KkContext) -> kk_string_t {
	let ptr = unsafe { kk_box_to_ptr::<Handle>(&h, ctx) };
	let h_ptr: *const Handle = &*ptr;
	println!("showing ptr {:?}", h_ptr);
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
