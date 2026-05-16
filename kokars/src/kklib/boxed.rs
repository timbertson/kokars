use libc::*;
use super::generated::*;
use super::context::*;
use super::uncounted::*;

extern crate alloc;

use alloc::boxed::Box;

#[allow(non_camel_case_types)]
pub type kk_free_fun_t = unsafe extern "C" fn(*mut c_void, *const kk_block_t, KkContext);

pub type KkFreeFun<T> = unsafe extern "C" fn(Box<T>, *const kk_block_t, KkContext);

unsafe extern "C" {
	fn kk_cptr_raw_box(free_fun: kk_free_fun_t, f: *mut c_void, _ctx: KkContext) -> kk_box_t;
	fn kk_box_to_ptr_c(b: Uncounted<kk_box_t>, _ctx: KkContext) -> *mut c_void;
}

pub fn kk_raw_box<T:Sized>(drop_fn: KkFreeFun<T>, value: Box<T>, _ctx: KkContext) -> kk_box_t {
	let box_ptr = Box::<T>::into_raw(value);
	unsafe {
		let untyped_drop_fn = core::mem::transmute::<KkFreeFun<T>, kk_free_fun_t>(drop_fn);
		kk_cptr_raw_box(untyped_drop_fn, box_ptr.cast::<c_void>(), _ctx)
	}
}

pub unsafe fn kk_box_to_ptr<'a, T>(b: &'a kk_box_t, ctx: KkContext) -> &'a T {
	unsafe {
		let ptr: *mut c_void = kk_box_to_ptr_c(Uncounted::unsafe_from_ref(b), ctx);
		core::mem::transmute::<*mut c_void, &'a T>(ptr)
	}
}
