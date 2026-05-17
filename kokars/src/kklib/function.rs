use super::generated::*;
use super::krc::*;
use super::context::*;

use core::marker::PhantomData;

pub type KkFunPtrVoid = unsafe extern "C" fn(Borrowed<Krc<kk_function_t>>, KkContext);
pub type KkFunPtr0<Ret> = unsafe extern "C" fn(Borrowed<Krc<kk_function_t>>, KkContext) -> Ret;
pub type KkFunPtr1<A, Ret> = unsafe extern "C" fn(Borrowed<Krc<kk_function_t>>, Krc<A>, KkContext) -> Ret;
pub type KkFunPtr2<A, B, Ret> = unsafe extern "C" fn(Borrowed<Krc<kk_function_t>>, Krc<A>, Krc<B>, KkContext) -> Ret;

unsafe extern "C" {
	fn kk_function_cptr_borrow_c<'a>(f: Borrowed<Krc<kk_function_t>>, _ctx: KkContext) -> KkFunPtrVoid;
}

#[repr(transparent)]
#[derive(Clone)]
pub struct KkFunction1<A: RefCounted, Ret: RefCounted> {
	untyped: Krc<kk_function_t>,
	_marker: PhantomData<KkFunPtr1<A, Ret>>,
}

impl<A: RefCounted, Ret: RefCounted> KkFunction1<A, Ret> {
	pub fn call(self, a: Krc<A>, _ctx: KkContext) -> Ret {
		unsafe {
			kk_function_call_1(self.untyped, a, _ctx)
		}
	}
}

unsafe fn drop_without_refcount<A>(a: A) {
	core::mem::forget(a);
}

pub unsafe fn kk_function_call_1<A: RefCounted, Ret: RefCounted>(f: Krc<kk_function_t>, a: Krc<A>, _ctx: KkContext) -> Ret {
	unsafe {
		let f_borrow = f.unsafe_borrow();
		// `f` is consumed when it is called, don't double-drop it
		drop_without_refcount(f);

		let raw_cfn = kk_function_cptr_borrow_c(f_borrow.unsafe_copy(), _ctx);
		let typed_cfn = core::mem::transmute::<KkFunPtrVoid, KkFunPtr1<A, Ret>>(raw_cfn);
		typed_cfn(f_borrow, a, _ctx)
	}
}
