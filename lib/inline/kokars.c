reexport_static1(kk_ssize_t, kk_to_ssize_t, size_t);
reexport_static1(size_t, kk_to_size_t, kk_ssize_t);

reexport_static2(void, kk_function_drop, kk_function_t, kk_context_t*);
reexport_static2(kk_function_t, kk_function_dup, kk_function_t, kk_context_t*);

reexport_static2(void, kk_string_drop, kk_string_t, kk_context_t*);
reexport_static2(kk_string_t, kk_string_dup, kk_string_t, kk_context_t*);

reexport_static2(void, kk_box_drop, kk_box_t, kk_context_t*);
reexport_static2(kk_box_t, kk_box_dup, kk_box_t, kk_context_t*);

reexport_static2(void, kk_integer_drop, kk_integer_t, kk_context_t*);
reexport_static2(kk_integer_t, kk_integer_dup, kk_integer_t, kk_context_t*);

reexport_static3(const char*, kk_string_cbuf_borrow, const kk_string_t, kk_ssize_t*, kk_context_t*);

reexport_static2(void, kk_free, char*, kk_context_t*);

typedef void (*void_fn_ptr)();

void_fn_ptr kk_function_cptr_borrow_c(kk_function_t f, kk_context_t* ctx) {
	return kk_kkfun_ptr_unbox(kk_datatype_as_assert(struct kk_function_s*, f, KK_TAG_FUNCTION,ctx)->fun, ctx);
}

void* kk_malloc_aligned_c(kk_ssize_t sz, kk_ssize_t alignment, kk_context_t* ctx) {
  return mi_theap_malloc_aligned(ctx->heap, (size_t)sz, (size_t) alignment);
}
