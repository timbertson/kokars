
fun main()
  println("#![allow(unused_doc_comments)]");
  println("use super::krc::*;");

  opaque_struct("kk_context", s_ctx(), can_box=False, assume_value_type=True)
  opaque_struct("kk_block", s_block(), can_box=False, assume_value_type=True)
  opaque_struct("kk_box", s_box(), can_box=False)

  println("unsafe extern \"C\" {");
  println("  pub fn kk_get_context() -> *const kk_context_t;");
  println("}");
  
  println("#[repr(C, align(" ++ max_align_size().int.show ++ "))]");
  println("pub struct KkAllocator;");

  opaque_struct("kk_ssize", s_ssize(), assume_value_type = True)
  opaque_struct("kk_string", s_string())
  opaque_struct("kk_function", s_function())
  opaque_struct("kk_integer", s_integer())

extern s_ctx(): ssize_t { c inline "sizeof(kk_context_t)" }
extern s_box(): ssize_t { c inline "sizeof(kk_box_t)" }
extern s_block(): ssize_t { c inline "sizeof(kk_block_t)" }
extern s_string(): ssize_t { c inline "sizeof(kk_string_t)" }
extern s_function(): ssize_t { c inline "sizeof(kk_function_t)" }
extern s_tag(): ssize_t { c inline "sizeof(kk_tag_t)" }
extern s_integer(): ssize_t { c inline "sizeof(kk_integer_t)" }
extern s_ssize(): ssize_t { c inline "sizeof(kk_ssize_t)" }

// These declarations exist for rust to understand existing C structs, don't
// try to override the real struct definitions in generated C code
val for_rust_only = "/// cbindgen:ignore"

extern max_align_size(): ssize_t { c inline "MI_MAX_ALIGN_SIZE" }

fun extern_c(str)
  ["unsafe extern \"C\" {", "pub " ++ str, "}" ].join("\n")

fun printlines(lines: list<string>)
  println(lines.join("\n"))

fun opaque_struct(name, size: ssize_t, can_box = True, wrap_drop = True, assume_value_type = False)
  val tname = name ++ "_t"
  val drop_fn = name ++ (if wrap_drop then "_drop_c" else "_drop");
  val dup_fn = name ++ (if wrap_drop then "_dup_c" else "_dup");
  printlines([
    "",
    for_rust_only,
    "#[repr(C)]",
    "pub struct " ++ tname ++ " {",
    "  _opaque: [u8;" ++ size.int.show ++ "],",
    "}",
  ])

  if !assume_value_type then
    if wrap_drop then
      println(for_rust_only)
      println(extern_c("fn " ++ drop_fn ++ "(v: Borrowed<Krc<" ++ tname ++ ">>, _ctx: *const kk_context_t);"));

      println(for_rust_only)
      println(extern_c("fn " ++ dup_fn ++ "(v: Borrowed<Krc<" ++ tname ++ ">>, _ctx: *const kk_context_t) -> Krc<" ++ tname ++ ">;"));

    printlines([
      for_rust_only,
      extern_c("fn " ++ name ++ "_dup(v: Borrowed<Krc<" ++ tname ++ ">>, _ctx: *const kk_context_t) -> Krc<" ++ tname ++ ">;"),

      for_rust_only,
      extern_c("fn " ++ name ++ "_drop(v: Borrowed<Krc<" ++ tname ++ ">>, _ctx: *const kk_context_t);"),

      // Implement RefCounted for all generated types
      "impl RefCounted for " ++ tname ++ "{",
      "  fn kk_incr(&self, ctx: *const kk_context_t) -> Krc<Self> {",
      // "    println!(\"[dup:" ++ tname ++ "]\");",
      "    Krc::incr_via(" ++ dup_fn ++ ", self, ctx)",
      "  }",
      "  fn kk_decr(&self, ctx: *const kk_context_t) {",
      // "    println!(\"[dup:" ++ tname ++ "]\");",
      "    Krc::decr_via(" ++ drop_fn ++ ", self, ctx)",
      "  }",
      "}",

      // // Clone takes a reference and returns a fresh value from *_dup()
      // "impl Clone for " ++ tname ++ "{",
      // "  fn clone(&self) -> Self {",
      // // "    println!(\"[dup:" ++ tname ++ "]\");",
      // "    unsafe {",
      // "      " ++ dup_fn ++ "(Borrowed::unsafe_from_ref(self), kk_get_context())",
      // "    }",
      // "  }",
      // "}",

      // // Drop performs a _drop on every koka value
      // "impl Drop for " ++ tname ++ "{",
      // "  fn drop(&mut self) {",
      // // "    println!(\"[drop:" ++ tname ++ "]\");",
      // "    unsafe {",
      //        // TODO can we pass in the ctx somehow?
      // "      " ++ drop_fn ++ "(Borrowed::unsafe_from_ref(self), kk_get_context())",
      // "    }",
      // "  }",
      // "}",
    ])

  if can_box then
    println(for_rust_only)
    println(extern_c("fn " ++ name ++ "_box(v: " ++ tname ++ ", _ctx: *const kk_context_t) -> kk_box_t;"))
