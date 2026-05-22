use libc::*;
use super::generated::*;
use super::context::*;
use super::krc::*;

use core::marker::PhantomData;
use core::ptr;
use core::fmt;

extern crate alloc;

use alloc::boxed::Box;

#[allow(non_camel_case_types)]
pub type kk_free_fun_t = unsafe extern "C" fn(*mut c_void, *const kk_block_t, KkContext);

pub type KkFreeFun<T> = unsafe extern "C" fn(Box<T>, *const kk_block_t, KkContext);

unsafe extern "C" {
	fn kk_cptr_raw_box(free_fun: kk_free_fun_t, f: *mut c_void, _ctx: KkContext) -> Krc<kk_box_t>;
	fn kk_cptr_raw_unbox_borrowed(b: Borrowed<Krc<kk_box_t>>, _ctx: KkContext) -> *mut c_void;
}

// KkBox<T> is the koka representation of a Box<T>, a heap-allocated T.
// The kk_box_t must be a raw cptr, this wrapper should
// not be used for other boxed types.
#[repr(transparent)]
#[derive(Clone)]
// TODO: rename KrcBox?
pub struct KkBox<T:Sized> {
	pub untyped: Krc<kk_box_t>,
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

	pub unsafe fn cast(untyped: Krc<kk_box_t>) -> KkBox<T> {
		KkBox { untyped, _marker: PhantomData }
	}
	
	pub fn as_ref<'a>(&'a self, ctx: KkContext) -> &'a T {
		unsafe {
			let ptr: *mut c_void = kk_cptr_raw_unbox_borrowed(self.untyped.unsafe_borrow(), ctx);
			core::mem::transmute::<*mut c_void, &'a T>(ptr)
		}
	}
}

// Like Krc<>::mutate_or_copy, but for typed, boxed values.
impl<T> KkBox<T> {
	pub fn mutate_or_copy<
		FM: FnOnce(&mut T, KkContext),
		FC: FnOnce(KkBox<T>, KkContext) -> KkBox<T>,
	>(self, mutate: FM, copy: FC, ctx: KkContext) -> KkBox<T> {
		if self.untyped.get_refcount(ctx) == 0 {
			unsafe {
				let borrow = self.untyped.unsafe_borrow();
				let ptr: *mut c_void = kk_cptr_raw_unbox_borrowed(borrow, ctx);
				let ref_t: &mut T = &mut *(ptr.cast::<T>());
				mutate(ref_t, ctx);
			};
			self
		} else {
			copy(self, ctx)
		}
	}
}


impl<T: fmt::Debug> fmt::Debug for KkBox<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		let ctx = unsafe { kk_get_context() };
		self.as_ref(ctx).fmt(f)
	}
}

// unsafe extern "C" {
// 	fn kk_block_refcount_c(p: *const c_void) -> kk_refcount_t;
// 	fn kk_box_to_ptr(b: Borrowed<Krc<kk_box_t>>, ctx: KkContext) -> *mut kk_block_t;
// }

// impl HasRefCount for Krc<kk_box_t> {
// 	fn get_refcount(&self, ctx: KkContext) -> kk_refcount_t {
// 		unsafe {
// 			let block: *mut kk_block_t = kk_box_to_ptr(self.unsafe_borrow(), ctx);
// 			kk_block_refcount_c(block as *const c_void)
// 		}
// 	}
// }

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
// reference counting logic will be incorrect.
#[repr(transparent)]
#[derive(Clone)]
pub struct KkBoxWrapper<T> {
	pub value: KkBox<T>,
}

// This adds a rust type with the same name as the generated C type
/// cbindgen:ignore
pub type kk_kokars__box_wrapper = KkBoxWrapper<c_void>;

unsafe extern "C" {
	// TODO expose these if necessary?
	// pub fn kk_kokars__box_wrapper_box(wrapper: Borrowed<kk_kokars__box_wrapper>, ctx: *const kk_context_t) -> kk_box_t;
	// pub fn kk_kokars__box_wrapper_unbox(wrapper: Borrowed<kk_box_t>, ctx: *const kk_context_t) -> kk_kokars__box_wrapper;

	// These would be exposed if the struct were heap allocated
	// pub fn kk_kokars__box_wrapper_dup(wrapper: Borrowed<Krc<kk_kokars__box_wrapper>>, ctx: *const kk_context_t) -> Krc<kk_kokars__box_wrapper>;
	// pub fn kk_kokars__box_wrapper_drop(wrapper: Borrowed<Krc<kk_kokars__box_wrapper>>, ctx: *const kk_context_t);
}

impl<T> KkBoxWrapper<T> {
	pub fn new(value: KkBox<T>) -> KkBoxWrapper<T> {
		KkBoxWrapper { value }
	}
	
	pub fn into_value(self) -> KkBox<T> {
		self.value
	}
	
	// pub unsafe fn unsafe_cast<R>(self) -> KkBoxWrapper<R> {
	// 	unsafe {
	// 		ptr::read((&self as *const KkBoxWrapper<T>) as *const KkBoxWrapper<R>)
	// 	}
	// }

	// pub unsafe fn unsafe_borrow(&self) -> Borrowed<Krc<KkBoxWrapper<c_void>>> {
	// 	Borrowed::unsafe_wrap_raw(ptr::read((self as *const KkBoxWrapper<T>) as *const KkBoxWrapper<c_void>))
	// }
}

impl<T> Krc<KkBoxWrapper<T>> {
	// pub unsafe fn c_void(&self) -> &Krc<KkBoxWrapper<c_void>> {
	// 	panic!()
	// }

	pub unsafe fn unsafe_cast<R>(self) -> Krc<KkBoxWrapper<R>> {
		unsafe {
			ptr::read((&self as *const Krc<KkBoxWrapper<T>>) as *const Krc<KkBoxWrapper<R>>)
		}
	}
}

impl<T> core::ops::Deref for KkBoxWrapper<T> {
	type Target = KkBox<T>;
	fn deref(&self) -> &Self::Target {
		&self.value
	}
}

// KkBoxWrapper is a value struct, use value semantics for RefCount
impl<T> RefCounted for KkBoxWrapper<T> {
	fn kk_incr(value: &Krc<Self>, _ctx: KkContext) -> Krc<Self> {
		unsafe { ptr::read(value) }
	}

	fn kk_decr(_value: &Krc<Self>, _ctx: KkContext) {}
}

// impl<T> HasRefCount for KkBoxWrapper<T> {
// 	fn get_refcount(&self, ctx: KkContext) -> kk_refcount_t {
// 		self.value.untyped.get_refcount(ctx)
// 	}
// }
