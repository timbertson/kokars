use super::uncounted::*;
use super::generated::*;
use super::context::*;

use core::fmt;
use core::marker::PhantomData;

pub type KkFunPtrVoid = unsafe extern "C" fn(Uncounted<kk_function_t>, KkContext);
pub type KkFunPtr0<Ret> = unsafe extern "C" fn(Uncounted<kk_function_t>, KkContext) -> Ret;
pub type KkFunPtr1<A, Ret> = unsafe extern "C" fn(Uncounted<kk_function_t>, A, KkContext) -> Ret;
pub type KkFunPtr2<A, B, Ret> = unsafe extern "C" fn(Uncounted<kk_function_t>, A, B, KkContext) -> Ret;

unsafe extern "C" {
	fn kk_function_cptr_borrow_c<'a>(f: Uncounted<kk_function_t>, _ctx: KkContext) -> KkFunPtrVoid;
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
	core::mem::forget(a);
}

pub unsafe fn kk_function_call_1<A,Ret>(f: kk_function_t, a: A, _ctx: KkContext) -> Ret {
	unsafe {
		let fvalue = Uncounted::unsafe_from_ref(&f);
		// `f` is consumed, however invoking `f` already decrements the refcount, we don't want to also drop it
		drop_without_refcount(f);

		let raw_cfn = kk_function_cptr_borrow_c(fvalue.uncounted_copy(), _ctx);
		let typed_cfn = core::mem::transmute::<KkFunPtrVoid, KkFunPtr1<A, Ret>>(raw_cfn);
		typed_cfn(fvalue, a, _ctx)
	}
}

impl fmt::Debug for kk_function_t {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		let ptr = unsafe { kk_function_cptr_borrow_c(Uncounted::unsafe_from_ref(self), kk_get_context()) };
		write!(f, "{:p}", ptr)
	}
}
