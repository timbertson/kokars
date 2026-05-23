#![no_std]
#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]

use kokars::kklib::*;
use libc_print::std_name::println;
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
	KkBoxWrapper::new(KkBox::new(Handle { id: i }, ctx))
}

// Use KkShow to implement a koka `show` function for any `Debug` type
#[unsafe(no_mangle)]
pub extern "C" fn kk_show_handle(h: Borrowed<Krc<kk_hello__rust_handle>>, ctx: KkContext) -> Krc<kk_string_t> {
	KkShow::show(&h, ctx)
}

// You might use this pattern if mutating is significantly cheaper (or more common) than copying.
// Note that this is best used on raw `any` types from koka. Destructuring the wrapper
// struct is more efficiently done in koka.
#[unsafe(no_mangle)]
pub extern "C" fn kk_handle_increment_id(h: Krc<KkBox<Handle>>, ctx: KkContext) -> Krc<KkBox<Handle>> {
	h.mutate_or_copy(|owned: &mut KkBox<Handle>, _ctx| {
		println!("Mutating handle: {:?}", owned);
		owned.id += 1;
	}, |shared: Krc<KkBox<Handle>>, ctx| {
		println!("Making a copy of handle: {:?}", shared);
		KkBox::new(Handle { id: shared.id + 1 }, ctx)
	}, ctx)
}

// Heap-allocated rust values can be stored in a Krc<KkBox<T>>.
// When the last reference to this value is dropped, the
// rust Drop implementation will be invoked.
impl Drop for Handle {
	fn drop(&mut self) {
		println!("Dropping handle: {}", self.id);
	}
}
