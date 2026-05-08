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

extern kkrs-callback1(s: string, f: (string) -> io-noexn int): io-noexn int
  c "kk_kkrs_callback1"

fun main()
  hello-kkrs()
  hello-kkrs("This is a Koka string")
  
  fun get-string-length(s: string)
    println("Getting length of string")
    println(s)
    s.count

  println("Getting the length of a string indirectly returned: " ++ kkrs-callback1("hello", get-string-length).show)
