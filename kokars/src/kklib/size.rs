pub use super::generated::*;

use libc::*;

unsafe extern "C" {
	pub fn kk_to_size_t_c(s: kk_ssize_t) -> size_t;
	pub fn kk_to_ssize_t_c(s: size_t) -> kk_ssize_t;
}
