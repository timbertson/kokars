use libc::*;
use super::generated::*;
use super::context::*;
use super::krc::*;
use super::size::*;

use core::marker::PhantomData;
use core::ptr;
use core::fmt;
use core::ops::Deref;

extern crate alloc;

use alloc::boxed::Box;

#[allow(non_camel_case_types)]
pub type kk_free_fun_t = unsafe extern "C" fn(*mut c_void, *const kk_block_t, KkContext);

pub type KkFreeFun<T> = unsafe extern "C" fn(Box<T>, *const kk_block_t, KkContext);

unsafe extern "C" {
	fn kk_cptr_raw_box(free_fun: kk_free_fun_t, f: *mut c_void, _ctx: KkContext) -> Krc<kk_box_t>;
	fn kk_cptr_raw_unbox_borrowed(b: Borrowed<Krc<kk_box_t>>, _ctx: KkContext) -> *mut c_void;
}

// KkBox<T> is the koka version of rust's Box<T>, a heap-allocated T.
// The underlying kk_box_t must be a raw cptr, this wrapper must
// not be used for other boxed types.
#[repr(transparent)]
pub struct KkBox<T:Sized> {
	pub untyped: kk_box_t,
	_marker: PhantomData<T>,
}

// mark KkBox<T> and kk_box_t as having the same koka representation
// (this allows safe casting)
impl<T> KkRepr<kk_box_t> for KkBox<T> {}
impl<T> KkRepr<KkBox<T>> for kk_box_t {}

impl<T:Sized> KkBox<T> {
	fn mut_ptr(&self) -> *mut T {
		unsafe {
			let ctx = kk_get_context();
			let direct_box: kk_box_t = ptr::read(&self.untyped);
			let borrowed_box: Borrowed<Krc<kk_box_t>> = Borrowed::unsafe_wrap_raw(Krc::unsafe_wrap_raw(direct_box));
			let ptr: *mut c_void = kk_cptr_raw_unbox_borrowed(borrowed_box, ctx);
			core::mem::transmute::<*mut c_void, *mut T>(ptr)
		}
	}
}

impl<T:Sized + KkDrop> KkBox<T> {
	pub fn new(value: T, _ctx: KkContext) -> Krc<KkBox<T>> {
		Self::wrap(Box::new(value), _ctx)
	}

	pub fn wrap(value: Box<T>, _ctx: KkContext) -> Krc<KkBox<T>> {
		let box_ptr = Box::<T>::into_raw(value);
		unsafe {
			let untyped_drop_fn = core::mem::transmute::<KkFreeFun<T>, kk_free_fun_t>(T::kk_drop);
			let untyped = kk_cptr_raw_box(untyped_drop_fn, box_ptr.cast::<c_void>(), _ctx);
			untyped.cast_repr()
		}
	}
}

impl<T> core::ops::Deref for KkBox<T> {
	type Target = T;
	fn deref<'a>(&'a self) -> &'a Self::Target {
		unsafe { &*self.mut_ptr() as &'a T }
	}
}

impl<T> core::ops::DerefMut for KkBox<T> {
	fn deref_mut<'a>(&'a mut self) -> &'a mut Self::Target {
		unsafe { &mut *self.mut_ptr() as &'a mut T }
	}
}

impl<T> RefCounted for KkBox<T> {
	fn kk_incr(b: &Krc<Self>, ctx: KkContext) -> Krc<Self> {
		unsafe {
			let untyped: &Krc<kk_box_t> = b.cast_repr_ref();
			RefCounted::kk_incr(untyped, ctx).cast_repr()
		}
	}

	fn kk_decr(b: &Krc<Self>, ctx: KkContext) {
		unsafe {
			RefCounted::kk_decr(b.cast_repr_ref::<kk_box_t>(), ctx)
		}
	}
}

impl<T> HasRefCount for KkBox<T> {
	fn get_refcount(&self, ctx: KkContext) -> kk_refcount_t {
		self.untyped.get_refcount(ctx)
	}
}

impl<T: fmt::Debug> fmt::Debug for KkBox<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		let inner: &T = self.deref();
		inner.fmt(f)
	}
}

// Commonly koka modules define a "newtype" wrapping an `any`, e.g:
//
// ```koka
// value struct my-handle
//   inner: any
// ```
//
// KkBoxWrapper<T> is just a convenient struct which matches that common memory layout,
// so you can use it in APIs accepting any struct with the same layout.
//
// Note that the struct **must** be a `value struct`, otherwise this type's
// memory management logic will be incorrect.
#[repr(transparent)]
#[derive(Clone)]
pub struct KkBoxWrapper<T> {
	pub value: Krc<KkBox<T>>,
}

// This adds a rust type with the same name as the generated C type
/// cbindgen:ignore
pub type kk_kokars__box_wrapper = KkBoxWrapper<c_void>;

impl<T> KkBoxWrapper<T> {
	pub fn new(value: Krc<KkBox<T>>) -> KkBoxWrapper<T> {
		KkBoxWrapper { value }
	}
	
	pub fn into_value(self) -> Krc<KkBox<T>> {
		self.value
	}
}

impl<T: fmt::Debug> fmt::Debug for KkBoxWrapper<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		let inner: &T = self.deref();
		inner.fmt(f)
	}
}

impl<T> core::ops::Deref for KkBoxWrapper<T> {
	type Target = KkBox<T>;
	fn deref(&self) -> &Self::Target {
		&self.value.deref()
	}
}

// KkBoxWrapper is a value struct, use value semantics for RefCount
impl<T> RefCounted for KkBoxWrapper<T> {
	fn kk_incr(value: &Krc<Self>, _ctx: KkContext) -> Krc<Self> {
		unsafe { ptr::read(value) }
	}

	fn kk_decr(_value: &Krc<Self>, _ctx: KkContext) {}
}

pub trait KkDrop {
	unsafe extern "C" fn kk_drop(h: Box<Self>, _block: *const kk_block_t, _ctx: KkContext);
}

#[allow(drop_bounds)]
impl<T: Drop> KkDrop for T {
	unsafe extern "C" fn kk_drop(value: Box<T>, _block: *const kk_block_t, _ctx: KkContext) {
		drop(value);
	}
}
