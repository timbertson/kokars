import kokars

extern import
  c { library="kokars_example" }

extern import
  c file "generated/rust.h"

// these externs are rust functions, exposed as a C ABI and
// defined in the generated header file
extern hello-kkrs(): io-noexn ()
  c "kk_hello_rs"

extern str/hello-kkrs(s: string): io-noexn ()
  c "kk_hello_rs_str"

fun main()
  hello-kkrs()
  hello-kkrs("This is a Koka string")
