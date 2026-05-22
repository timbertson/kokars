#![no_std]
#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]

extern crate alloc;

use kokars::kklib::*;
use libc_print::std_name::println;
use alloc::boxed::Box;
use alloc::format;
use core::fmt;

#[unsafe(no_mangle)]
pub extern "C" fn kk_hello_rs(_c: KkContext) {
	println!("Hello from Rust!");
}

#[unsafe(no_mangle)]
pub extern "C" fn kk_hello_rs_str(kk_str: Krc<kk_string_t>, ctx: KkContext) {
	let str: &str = kk_string_as_str(&kk_str, ctx);
	// drop(kk_str); // wouldn't compile!
	println!("koka-string:[[{}]]", str);
}

#[unsafe(no_mangle)]
pub extern "C" fn kk_kkrs_callback1(kk_str: Krc<kk_string_t>, cb: Krc<kk_function_t>, ctx: KkContext) -> kk_integer_t {
	println!("calling function with stringval {}", kk_string_as_str(&kk_str, ctx));
	unsafe {
		kk_function_call_1::<kk_string_t, kk_integer_t>(cb, kk_str, ctx)
	}
}

#[repr(C)]
pub struct Handle {
	id: i32,
}

impl fmt::Debug for Handle {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		write!(f, "Handle({})", self.id)
	}
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

// TODO could this be derived?
#[unsafe(no_mangle)]
pub extern "C" fn kk_show_handle(h: Borrowed<Krc<kk_hello__rust_handle>>, ctx: KkContext) -> Krc<kk_string_t> {
	let shown = format!("{:?}", &h.value.as_ref(ctx));
	kk_string_alloc_dup_valid_utf8(&shown, ctx)
}

// You might use this pattern if mutating is significantly cheaper (or more common) than copying.
// Note that rust doesn't have easy access to construct a new KkBoxWrapper
#[unsafe(no_mangle)]
pub extern "C" fn kk_handle_increment_id(h: KkBox<Handle>, ctx: KkContext) -> KkBox<Handle> {
	h.mutate_or_copy(|owned: &mut Handle, _ctx| {
		println!("Mutating handle: {:?}", owned);
		owned.id += 1;
	}, |shared: KkBox<Handle>, ctx| {
		let old_handle: &Handle = shared.as_ref(ctx);
		println!("Making a copy of handle: {:?}", &old_handle);
		KkBox::new(kk_free_handle, Handle { id: old_handle.id + 1 }, ctx)
	}, ctx)
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
