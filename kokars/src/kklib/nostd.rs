/*
NOTE: final libraries must also be built with:

[profile.dev]
panic="abort"

[profile.release]
panic="abort"

 */

use super::generated::*;
use super::size::*;

use libc_print::std_name::eprintln;
// use core::ptr;
use core::alloc::*;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	eprintln!("{}", info);
	unsafe {
		libc::exit(100);
	}
}

// Requred for custom panic handler
#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}



// Allocation support
unsafe extern "C" {
	pub fn kk_malloc_aligned_c(sz: kk_ssize_t, alignment: kk_ssize_t, ctx: *const kk_context_t) -> *mut u8;
	pub fn kk_free_c(ptr: *mut u8, ctx: *const kk_context_t);
}

#[global_allocator]
static ALLOCATOR: KkAllocator = KkAllocator;

unsafe impl Sync for KkAllocator {}

unsafe impl GlobalAlloc for KkAllocator {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		unsafe {
			let size = kk_to_ssize_t_c(layout.size());
			let align = kk_to_ssize_t_c(layout.align());
			return kk_malloc_aligned_c(size, align, kk_get_context());
		}
	}
	unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
		unsafe {
			kk_free_c(ptr, kk_get_context());
		}
	}
}
