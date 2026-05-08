mod generated;
mod uncounted;

pub use generated::*;
pub use uncounted::*;

use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::slice;
use std::fmt;
use libc::*;

pub type KkContext = *const kk_context_t;
pub type KkFunPtrVoid = unsafe extern "C" fn(Uncounted<kk_function_t>, KkContext);
pub type KkFunPtr0<Ret> = unsafe extern "C" fn(Uncounted<kk_function_t>, KkContext) -> Ret;
pub type KkFunPtr1<A, Ret> = unsafe extern "C" fn(Uncounted<kk_function_t>, A, KkContext) -> Ret;
pub type KkFunPtr2<A, B, Ret> = unsafe extern "C" fn(Uncounted<kk_function_t>, A, B, KkContext) -> Ret;

unsafe extern "C" {
	fn kk_string_cbuf_borrow_c(str: Uncounted<kk_string_t>, len: *mut kk_ssize_t, _ctx: KkContext) -> *const u8;
	fn kk_to_size_t_c(s: kk_ssize_t) -> size_t;

	fn kk_function_cptr_borrow_c<'a>(f: Uncounted<kk_function_t>, _ctx: KkContext) -> KkFunPtrVoid;
}

pub fn kk_string_as_str<'a>(kk_str: &'a kk_string_t, _ctx: KkContext) -> &'a str {
	let mut len = MaybeUninit::<kk_ssize_t>::uninit();
	unsafe {
		let raw_ptr = kk_string_cbuf_borrow_c(Uncounted::unsafe_from_ref(kk_str), (&mut len).as_mut_ptr(), _ctx);
		let rlen = kk_to_size_t_c(len.assume_init());
		str::from_utf8_unchecked(slice::from_raw_parts(raw_ptr, rlen))
	}
}

#[repr(transparent)]
pub struct KkFunction1<A, Ret> {
	untyped: kk_function_t,
	_marker: PhantomData<KkFunPtr1<A, Ret>>,
}

impl<A,Ret> KkFunction1<A, Ret> {
	pub fn call(self, a: A, _ctx: KkContext) -> Ret {
		unsafe {
			kk_function_call_1(self.untyped, a, _ctx)
		}
	}
}

unsafe fn drop_without_refcount<A>(a: A) {
	std::mem::forget(a);
}

pub unsafe fn kk_function_call_1<A,Ret>(f: kk_function_t, a: A, _ctx: KkContext) -> Ret {
	unsafe {
		let fvalue = Uncounted::unsafe_from_ref(&f);
		// `f` is consumed, however invoking `f` already decrements the refcount, we don't want to also drop it
		drop_without_refcount(f);

		let raw_cfn = kk_function_cptr_borrow_c(fvalue.clone(), _ctx);
		let typed_cfn = std::mem::transmute::<KkFunPtrVoid, KkFunPtr1<A, Ret>>(raw_cfn);
		typed_cfn(fvalue, a, _ctx)
	}
}

impl fmt::Debug for kk_function_t {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		let ptr = unsafe { kk_function_cptr_borrow_c(Uncounted::unsafe_from_ref(self), kk_get_context()) };
		write!(f, "{:p}", ptr)
	}
}

