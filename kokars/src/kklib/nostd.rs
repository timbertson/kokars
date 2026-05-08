use libc_print::std_name::eprintln;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	eprintln!("{}", info);
	unsafe {
		libc::exit(100);
	}
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}

/*
NOTE: final libraries must also be built with:

[profile.dev]
panic="abort"

[profile.release]
panic="abort"

 */
