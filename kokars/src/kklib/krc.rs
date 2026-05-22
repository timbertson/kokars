use core::ops::Drop;
use core::clone::Clone;
use core::ptr;
use core::mem::ManuallyDrop;
use core::cell::UnsafeCell;
use libc::*;

use super::size::*;
use super::context::*;

pub trait RefCounted : Sized {
	fn kk_incr(value: &Krc<Self>, ctx: KkContext) -> Krc<Self>;
	fn kk_decr(value: &Krc<Self>, ctx: KkContext);
}

// This is separate to `RefCounted` since
// objects which are not within a Krc<T>
// can implement HasRefCount via delgation.
pub trait HasRefCount {
	fn get_refcount(&self, ctx: KkContext) -> kk_refcount_t;
}

/*
 * Krc<T> represents all standard koka refcounted values.
 * Clone and Drop implementations integrate with koka's `dup` and `drop` functionality.
 *
 * Note that `T` should *not* have its own Drop implementation, the value should only be
 * freed / finalized when the koka ref drops to zero.
 *
 * All koka functions of type `a -> b` should be typed in rust as `Krc<a> -> Krc<b>`
 */
#[repr(transparent)]
pub struct Krc<T: RefCounted> {
	value: UnsafeCell<T>
}

impl<T: RefCounted> Krc<T> {
	pub unsafe fn unsafe_wrap_raw(value: T) -> Self {
		Krc { value: UnsafeCell::new(value) }
	}

	unsafe fn unsafe_borrow_raw(value: &UnsafeCell<T>) -> Borrowed<Krc<T>> {
		unsafe {
			let p : *const T = value.get();
			let raw = ptr::read(p);
			Borrowed::unsafe_wrap_raw(Self::unsafe_wrap_raw(raw))
		}
	}

	pub unsafe fn unsafe_borrow(&self) -> Borrowed<Krc<T>> {
		unsafe { Self::unsafe_borrow_raw(&self.value) }
	}

	// helpers for implementing RefCounted using koka-generated C functions
	pub fn incr_via(f: unsafe extern "C" fn(value: Borrowed<Krc<T>>, ctx: KkContext) -> Krc<T>, value: &Krc<T>, ctx: KkContext) -> Krc<T> {
		unsafe { f(value.unsafe_borrow(), ctx) }
	}
	pub fn decr_via(f: unsafe extern "C" fn(value: Borrowed<Krc<T>>, ctx: KkContext), value: &Krc<T>, ctx: KkContext) {
		unsafe { f(value.unsafe_borrow(), ctx) }
	}
}

impl<T: HasRefCount + RefCounted> Krc<T> {
	pub fn mutate_or_copy<
		FM: FnOnce(&mut T, KkContext),
		FC: FnOnce(Krc<T>, KkContext) -> Krc<T>,
	>(self, mutate: FM, copy: FC, ctx: KkContext) -> Krc<T> {
		if self.get_refcount(ctx) == 0 {
			let inner: &mut T = unsafe {
				&mut *self.value.get()
			};
			mutate(inner, ctx);
			self
		} else {
			copy(self, ctx)
		}
	}
}

impl<T: RefCounted> Clone for Krc<T> {
	fn clone(&self) -> Krc<T> {
		unsafe {
			T::kk_incr(self, kk_get_context())
		}
	}
}

impl<T: RefCounted> Drop for Krc<T> {
	fn drop(&mut self) {
		unsafe {
			T::kk_decr(self, kk_get_context());
		}
	}
}

impl<T: RefCounted> core::ops::Deref for Krc<T> {
	type Target = T;
	fn deref(&self) -> &Self::Target {
		unsafe { & *self.value.get() }
	}
}

/*
 * Borrowed<T> is a wrapper around a value to exclude it from reference counting.
 * Functionally, it's an alias of `ManuallyDrop`.
 *
 * It should only be used for:
 *  - borrowed parameters (e.g. foo(^a): ())
 *  - passing to low-level C functions that operate on a borrowed value
 *    (these typically include `borrow` in the function name).
 */
// TODO: add lifetime annotation to make this safe?
#[repr(transparent)]
pub struct Borrowed<T> {
	value: ManuallyDrop<T>
}

impl<T> Borrowed<T> {
	// pub fn clone(&self) -> T {
	// 	ManuallyDrop::into_inner(self.value.clone())
	// }
	
	pub unsafe fn unsafe_wrap_raw(value: T) -> Self {
		Borrowed { value: ManuallyDrop::new(value) }
	}

	pub unsafe fn unsafe_wrap_ref(value: &T) -> Self {
		unsafe {
			Borrowed { value: ManuallyDrop::new(ptr::read(value)) }
		}
	}
}

impl<T: RefCounted> Borrowed<Krc<T>> {
	pub unsafe fn unsafe_copy(&self) -> Borrowed<Krc<T>> {
		unsafe { Krc::unsafe_borrow(&self.value) }
	}
}

impl<T> core::ops::Deref for Borrowed<T> {
	type Target = T;

	fn deref(&self) -> &T {
		&self.value
	}
}


unsafe extern "C" {
	fn kk_block_refcount_c(p: *const c_void) -> kk_refcount_t;
	fn kk_box_to_ptr_c(b: Borrowed<Krc<kk_box_t>>, ctx: KkContext) -> *mut kk_block_t;
	fn kk_box_is_ptr_c(b: kk_box_t, ctx: KkContext) -> bool;
}

impl HasRefCount for kk_box_t {
	fn get_refcount(&self, ctx: KkContext) -> kk_refcount_t {
		unsafe {
			let self_ptr = self as *const kk_box_t;
			if kk_box_is_ptr_c(ptr::read(self_ptr), ctx) {
				let borrow: Borrowed<Krc<kk_box_t>> = Borrowed::unsafe_wrap_raw(Krc::unsafe_wrap_raw(ptr::read(self_ptr)));
				let block: *mut kk_block_t = kk_box_to_ptr_c(borrow, ctx);
				kk_block_refcount_c(block as *const c_void)
			} else {
				return 0 // no values have multiple references; all boxes are either values or pointers
			}
		}
	}
}

// /*
//  * Unique<T> represents a Krc<T> where the reference count is 0, i.e. this is
//  * the only copy. Such values can be mutated safely without copying.
//  */
// pub struct Unique<T: RefCounted> {
// 	value: Krc<T>
// }

// impl<T: RefCounted> Unique<T> {
// 	// go back to a shared value
// 	pub fn shared(self) -> Krc<T> {
// 		self.value
// 	}
// }

// impl<T: RefCounted> core::ops::Deref for Unique<T> {
// 	type Target = T;

// 	fn deref(&self) -> &T {
// 		&self.value.deref()
// 	}
// }

// impl<T: RefCounted> core::ops::DerefMut for Unique<T> {
// 	fn deref_mut(&mut self) -> &mut T {
// 		&mut *self.value.value.get_mut()
// 	}
// }
