use core::mem::ManuallyDrop;
use core::ptr;

/*
 * A copy of an owned value which is exempt
 * from reference counting. clone() is just a value copy,
 * and drop() is a no-op.
 *
 * This should only be used when needing to bypass refcounting, often
 * when using C apis that provide some borrowed value for temporary use.
 */
#[repr(transparent)]
pub struct Uncounted<T> {
	inner: ManuallyDrop<T>,
}

impl<T> Uncounted<T> {
	pub unsafe fn unsafe_from_ref(value: &T) -> Uncounted<T> {
		unsafe {
			// raw copy of value's bytes without clone or drop
			let p : *const T = value;
			let inner = ManuallyDrop::new(ptr::read(p));
			Uncounted { inner }
		}
	}

	// We don't implement the Copy trait, since this method is unsafe
	pub unsafe fn uncounted_copy(&self) -> Uncounted<T> {
		unsafe {
			Uncounted::unsafe_from_ref(&self.inner)
		}
	}
}
