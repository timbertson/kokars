mod generated;
pub use generated::*;

use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::mem::MaybeUninit;
use std::ptr;
use std::slice;
use libc::*;

/*
 * A copy of an owned value. Acts like a reference
 * except that the underlying bytes are copied.
 *
 * This is used for C APIs which accept a value without
 * conforming to koka's typical ownership rules. Typically
 * these are named `*_borrow`, e.g. `kk_string_cbuf_borrow`
 *
 * A BarrowedValue must not outlive the value it's borrowing,
 * or else the value will become invalid. For simple koka value
 * types there is no impact, but anything with a pointer will
 * be pointing to invalid memory after the original value is dropped.
 */
#[repr(transparent)]
struct BorrowedValue<'a, T> {
	inner: ManuallyDrop<T>,
	_marker: PhantomData<&'a T>,
}

impl<'a, T> BorrowedValue<'a, T> {
	pub unsafe fn unsafe_borrow(value: &'a T) -> BorrowedValue<'a, T> {
		unsafe {
			// raw copy of value's bytes without clone or drop
			let p : *const T = value;
			let inner = ManuallyDrop::new(ptr::read(p));
			BorrowedValue { inner, _marker: PhantomData }
		}
	}
}

unsafe extern "C" {
	fn kk_string_cbuf_borrow_c<'a>(str: BorrowedValue<'a, kk_string_t>, len: *mut kk_ssize_t, _ctx: &kk_context_t) -> *const u8;
	// fn kk_to_ssize_t_c(s: size_t) -> kk_ssize_t;
	fn kk_to_size_t_c(s: kk_ssize_t) -> size_t;
}

pub fn kk_string_as_str<'a>(kk_str: &'a kk_string_t, _ctx: &kk_context_t) -> &'a str {
	let mut len = MaybeUninit::<kk_ssize_t>::uninit();
	unsafe {
		let borrow = BorrowedValue::unsafe_borrow(kk_str);
		let raw_ptr = kk_string_cbuf_borrow_c(borrow, (&mut len).as_mut_ptr(), _ctx);
		let rlen = kk_to_size_t_c(len.assume_init());
		str::from_utf8_unchecked(slice::from_raw_parts(raw_ptr, rlen))
	}
}
