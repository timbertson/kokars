
fun main()
  opaque_struct("kk_context", s_ctx(), minimal=True)
  opaque_struct("kk_box", s_box(), minimal=True)

  opaque_struct("kk_ssize", s_box())
  opaque_struct("kk_string", s_string())
  opaque_struct("kk_function", s_function())

extern s_ctx(): ssize_t
  c inline "sizeof(kk_context_t)"

extern s_box(): ssize_t
  c inline "sizeof(kk_box_t)"

extern s_string(): ssize_t
  c inline "sizeof(kk_string_t)"

extern s_function(): ssize_t
  c inline "sizeof(kk_function_t)"

fun extern_c(str)
  ["unsafe extern \"C\" {", "pub " ++ str, "}" ].join("\n")

fun opaque_struct(name, size: ssize_t, minimal = False)
  val tname = name ++ "_t"
  println([
    "#[repr(C)]",
    "pub struct " ++ tname ++ " {",
    "  _opaque: [u8;" ++ size.int.show ++ "],",
    "}",
    extern_c("fn " ++ name ++ "_dup(v: " ++ tname ++ ", _ctx: kk_context_t) -> " ++ tname ++ ";"),
    extern_c("fn " ++ name ++ "_drop(v: " ++ tname ++ ", _ctx: kk_context_t);"),
  ].join("\n"))
  
  if ! minimal then
    println(extern_c("fn " ++ name ++ "_box(v: " ++ tname ++ ", _ctx: kk_context_t) -> kk_box_t;"))
