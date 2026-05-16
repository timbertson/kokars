use core::ops::Drop;
use core::clone::Clone;
use core::ptr;
use core::mem::ManuallyDrop;

use super::size::*;
use super::context::*;

pub trait RefCounted : Sized {
	fn kk_dup(&self, ctx: KkContext) -> Krc<Self>;
	fn kk_drop(&self, ctx: KkContext);
}

/*
 * Krc<T> represents all standard koka refcounted values.
 * Clone and Drop implementations integrate with koka's `dup` and `drop` functionality.
 *
 * All koka functions of type `a -> b` should be typed in rust as `Krc<a> -> Krc<b>`
 */
#[repr(transparent)]
pub struct Krc<T: RefCounted> {
	value: ManuallyDrop<T>
}

impl<T: RefCounted> Krc<T> {
	unsafe fn wrap(value: ManuallyDrop<T>) -> Self {
		Krc { value }
	}

	unsafe fn unsafe_borrow_raw(value: &T) -> Borrowed<T> {
		unsafe {
			let p : *const T = value;
			let raw = ptr::read(p);
			Borrowed { value: ManuallyDrop::new(raw) }
		}
	}

	pub fn dup_via(f: extern "C" fn(value: Borrowed<T>, ctx: KkContext) -> Krc<T>, value: &T, ctx: KkContext) -> Krc<T> {
		unsafe { f(Self::unsafe_borrow_raw(value), ctx) }
	}
	
	pub unsafe fn unsafe_borrow(&self) -> Borrowed<T> {
		unsafe { Self::unsafe_borrow_raw(&self.value) }
	}
}

impl<T: RefCounted> Clone for Krc<T> {
	fn clone(&self) -> Krc<T> {
		unsafe {
			self.value.kk_dup(kk_get_context())
		}
	}
}

impl<T: RefCounted> Drop for Krc<T> {
	fn drop(&mut self) {
		unsafe {
			self.value.kk_drop(kk_get_context());
		}
	}
}

impl<T: RefCounted> core::ops::Deref for Krc<T> {
	type Target = T;
	fn deref(&self) -> &Self::Target {
		&self.value
	}
}


/*
 * Borrowed<T> is similar to Krc<T> but no reference counting occurs.
 * It should only be used for:
 *  - borrowed parameters (e.g. foo(^a): ())
 *  - passing to low-level C functions that operate on a borrowed value
 *    (these typically include `borrow` in the function name).
 */
// TODO: add lifetime annotation to make this safe
#[repr(transparent)]
pub struct Borrowed<T: RefCounted> {
	value: ManuallyDrop<T>
}

impl<T: RefCounted> Borrowed<T> {
	pub fn clone(&self) -> Krc<T> {
		unsafe {
			self.value.kk_dup(kk_get_context())
		}
	}
	
	unsafe fn into_inner(self) -> ManuallyDrop<T> {
		self.value
	}
}

/*
 * Unique<T> represents a Krc<T> where the reference count is 0, i.e. this is
 * the only copy. Such values can be mutated safely without copying.
 */
pub struct Unique<T: RefCounted> {
	value: Krc<T>
}

impl<T: RefCounted> Unique<T> {
	pub fn replace<R: RefCounted, F: FnOnce(ManuallyDrop<T>) -> R>(self, f: F) -> Unique<R> {
		unsafe {
			let replacement = f(self.value.unsafe_borrow().into_inner());
			Unique { value: Krc::wrap(ManuallyDrop::new(replacement)) }
		}
	}
}

impl<T: RefCounted> core::ops::Deref for Unique<T> {
	type Target = T;

	fn deref(&self) -> &T {
		&self.value.value
	}
}

impl<T: RefCounted> core::ops::DerefMut for Unique<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.value.value
	}
}

impl<T: RefCounted> Into<Krc<T>> for Unique<T> {
	// go back to a possbly-shared value
	fn into(self) -> Krc<T> {
		self.value
	}
}

/*
 * MaybeOwned<T> represents either a Shared or Owned koka value
 */
pub enum MaybeOwned<T: RefCounted> {
	Shared(Krc<T>),
	Owned(Unique<T>),
}

impl<T: RefCounted> MaybeOwned<T> {
	// TODO should ToOwned be a trait?
	pub fn to_owned<F: FnOnce(Krc<T>) -> Unique<T>>(self, clone_fn: F) -> Unique<T> {
		match self {
			Self::Shared(rc) => clone_fn(rc),
			Self::Owned(x) => x,
		}
	}
}

pub trait HasRefCount : RefCounted {
	fn get_refcount(&self, ctx: KkContext) -> kk_refcount_t;
}

impl<T: HasRefCount> MaybeOwned<T> {
	pub fn new(value: Krc<T>, ctx: KkContext) -> MaybeOwned<T> {
		if value.get_refcount(ctx) == 0 {
			MaybeOwned::Owned(Unique { value })
		} else {
			MaybeOwned::Shared(value)
		}
	}
}
