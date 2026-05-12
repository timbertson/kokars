
fun main()
  println("use super::uncounted::Uncounted;");

  opaque_struct("kk_context", s_ctx(), can_box=False, assume_value_type=True)
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
extern s_string(): ssize_t { c inline "sizeof(kk_string_t)" }
extern s_function(): ssize_t { c inline "sizeof(kk_function_t)" }
extern s_tag(): ssize_t { c inline "sizeof(kk_tag_t)" }
extern s_integer(): ssize_t { c inline "sizeof(kk_integer_t)" }
extern s_ssize(): ssize_t { c inline "sizeof(kk_ssize_t)" }

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
    "#[repr(C)]",
    "pub struct " ++ tname ++ " {",
    "  _opaque: [u8;" ++ size.int.show ++ "],",
    "}",
  ])

  if !assume_value_type then
    if wrap_drop then
      println(extern_c("fn " ++ drop_fn ++ "(v: Uncounted<" ++ tname ++ ">, _ctx: *const kk_context_t);"));
      println(extern_c("fn " ++ dup_fn ++ "(v: Uncounted<" ++ tname ++ ">, _ctx: *const kk_context_t) -> " ++ tname ++ ";"));

    printlines([
      extern_c("fn " ++ name ++ "_dup(v: Uncounted<" ++ tname ++ ">, _ctx: *const kk_context_t) -> " ++ tname ++ ";"),
      extern_c("fn " ++ name ++ "_drop(v: Uncounted<" ++ tname ++ ">, _ctx: *const kk_context_t);"),
      // Clone takes a reference and returns a fresh value from *_dup()
      "impl Clone for " ++ tname ++ "{",
      "  fn clone(&self) -> Self {",
      // "    println!(\"[dup:" ++ tname ++ "]\");",
      "    unsafe {",
      "      " ++ dup_fn ++ "(Uncounted::unsafe_from_ref(self), kk_get_context())",
      "    }",
      "  }",
      "}",

      // Drop performs a _drop on every koka value
      "impl Drop for " ++ tname ++ "{",
      "  fn drop(&mut self) {",
      // "    println!(\"[drop:" ++ tname ++ "]\");",
      "    unsafe {",
             // TODO can we pass in the ctx somehow?
      "      " ++ drop_fn ++ "(Uncounted::unsafe_from_ref(self), kk_get_context())",
      "    }",
      "  }",
      "}",
    ])

  if can_box then
    println(extern_c("fn " ++ name ++ "_box(v: " ++ tname ++ ", _ctx: *const kk_context_t) -> kk_box_t;"))
