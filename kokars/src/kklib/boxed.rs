use libc::*;
use super::generated::*;
use super::context::*;
use super::uncounted::*;
use core::marker::PhantomData;

extern crate alloc;

use alloc::boxed::Box;

#[allow(non_camel_case_types)]
pub type kk_free_fun_t = unsafe extern "C" fn(*mut c_void, *const kk_block_t, KkContext);

pub type KkFreeFun<T> = unsafe extern "C" fn(Box<T>, *const kk_block_t, KkContext);

unsafe extern "C" {
	fn kk_cptr_raw_box(free_fun: kk_free_fun_t, f: *mut c_void, _ctx: KkContext) -> kk_box_t;
	fn kk_cptr_raw_unbox_borrowed(b: Uncounted<kk_box_t>, _ctx: KkContext) -> *mut c_void;
}

// KkBox<T> is the koka representation of a Box<T>.
// The kk_box_t must be a raw cptr, this wrapper should
// not be used for other boxed types.
#[repr(transparent)]
#[derive(Clone)]
pub struct KkBox<T:Sized> {
	untyped: kk_box_t,
	_marker: PhantomData<T>,
}

impl<T:Sized> KkBox<T> {
	pub fn new(drop_fn: KkFreeFun<T>, value: T, _ctx: KkContext) -> KkBox<T> {
		Self::wrap(drop_fn, Box::new(value), _ctx)
	}

	pub fn wrap(drop_fn: KkFreeFun<T>, value: Box<T>, _ctx: KkContext) -> KkBox<T> {
		let box_ptr = Box::<T>::into_raw(value);
		unsafe {
			let untyped_drop_fn = core::mem::transmute::<KkFreeFun<T>, kk_free_fun_t>(drop_fn);
			KkBox::<T> {
				untyped: kk_cptr_raw_box(untyped_drop_fn, box_ptr.cast::<c_void>(), _ctx),
				_marker: PhantomData,
			}
		}
	}
	
	pub fn as_ref<'a>(&'a self, ctx: KkContext) -> &'a T {
		unsafe {
			let ptr: *mut c_void = kk_cptr_raw_unbox_borrowed(Uncounted::unsafe_from_ref(&self.untyped), ctx);
			core::mem::transmute::<*mut c_void, &'a T>(ptr)
		}
	}
}

// Commonly koka modules define a "newtype" wrapping an `any`, e.g:
//
// ```koka
// struct my-handle
//   inner: any
// ```
//
// KkBoxWrapper<T> is just a convenient struct which matches that common memory layout,
// so you can use it in APIs accepting any struct with the same layout.
#[repr(C)]
#[derive(Clone)]
pub struct KkBoxWrapper<T> {
	pub value: KkBox<T>,
}

impl<T> KkBoxWrapper<T> {
	pub fn new(value: KkBox<T>) -> KkBoxWrapper<T> {
		KkBoxWrapper { value }
	}
}

impl<T> core::ops::Deref for KkBoxWrapper<T> {
	type Target = KkBox<T>;
	fn deref(&self) -> &Self::Target {
		&self.value
	}
}
