use super::generated::*;
use super::context::*;
use super::uncounted::*;
use super::size::*;

use core::slice;
use core::mem::MaybeUninit;

unsafe extern "C" {
	fn kk_string_cbuf_borrow_c(str: Uncounted<kk_string_t>, len: *mut kk_ssize_t, _ctx: KkContext) -> *const u8;
}

pub fn kk_string_as_str<'a>(kk_str: &'a kk_string_t, _ctx: KkContext) -> &'a str {
	let mut len = MaybeUninit::<kk_ssize_t>::uninit();
	unsafe {
		let raw_ptr = kk_string_cbuf_borrow_c(Uncounted::unsafe_from_ref(kk_str), (&mut len).as_mut_ptr(), _ctx);
		let rlen = kk_to_size_t_c(len.assume_init());
		str::from_utf8_unchecked(slice::from_raw_parts(raw_ptr, rlen))
	}
}
