/*
 * A number of useful functionality is exposed only as static functions in a header file.
 * Reexport them as regular functions here so that rust can access them.
 *
 * TODO does this impact performance?
 */
const char* kk_string_cbuf_borrow_c(const kk_string_t str, kk_ssize_t* len, kk_context_t* ctx) {
	return kk_string_cbuf_borrow(str, len, ctx);
}

kk_ssize_t kk_to_ssize_t_c(size_t s) {
	return kk_to_ssize_t(s);
}

size_t kk_to_size_t_c(kk_ssize_t s) {
	return kk_to_size_t(s);
}
