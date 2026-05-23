extern crate alloc;

use super::generated::*;
use super::context::*;
use super::krc::*;
use super::size::*;

use alloc::format;
use core::fmt;
use core::slice;
use core::mem::MaybeUninit;

unsafe extern "C" {
	fn kk_string_cbuf_borrow_c(str: Borrowed<Krc<kk_string_t>>, len: *mut kk_ssize_t, _ctx: KkContext) -> *const u8;
	fn kk_string_alloc_dupn_valid_utf8_c(len: kk_ssize_t, str: *const u8, _ctx: KkContext) -> Krc<kk_string_t>;
}

pub fn kk_string_as_str<'a>(kk_str: &'a Krc<kk_string_t>, _ctx: KkContext) -> &'a str {
	let mut len = MaybeUninit::<kk_ssize_t>::uninit();
	unsafe {
		let raw_ptr = kk_string_cbuf_borrow_c(kk_str.unsafe_borrow(), (&mut len).as_mut_ptr(), _ctx);
		let rlen = kk_to_size_t_c(len.assume_init());
		str::from_utf8_unchecked(slice::from_raw_parts(raw_ptr, rlen))
	}
}

pub fn kk_string_alloc_dup_valid_utf8<'a>(str: &'a str, ctx: KkContext) -> Krc<kk_string_t> {
	unsafe {
		kk_string_alloc_dupn_valid_utf8_c(kk_to_ssize_t_c(str.len()), str.as_ptr(), ctx)
	}
}

pub trait KkShow {
	fn show(value: &Krc<Self>, ctx: KkContext) -> Krc<kk_string_t>
		where Self: RefCounted;
}

impl<T: RefCounted + fmt::Debug> KkShow for T {
	fn show(value: &Krc<Self>, ctx: KkContext) -> Krc<kk_string_t> {
		let inner: &Self = &*value;
		let shown = format!("{:?}", inner);
		kk_string_alloc_dup_valid_utf8(&shown, ctx)
	}
}
