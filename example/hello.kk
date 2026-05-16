import kokars

import std/num/int32
import std/num/int64

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

struct rust-handle {
  internal: any
}

extern kkrs-generate-handle(i: int32): any
  c "kk_generate_handle"

fun rust-handle(i: int) Rust-handle(kkrs-generate-handle(i.int32))

extern show-handle-inner(h: any): string
  c "kk_show_handle"

fun rust-handle/show(h: rust-handle): string
  show-handle-inner(h.internal)

extern get-memory-address(^box: any): io-noexn int64
  c inline "(long)kk_box_to_ptr(box, _ctx)"

fun section(desc)
  println("\n ===== " ++ desc ++ " =====")

noinline fun consume(v) ()

fun main()
  section("hello world")
  hello-kkrs()
  hello-kkrs("This is a Koka string")
  
  fun get-string-length(s: string)
    println("Getting length of string: " ++ s)
    s.count

  section("functions")
  println("Getting the length of a string indirectly returned: " ++ kkrs-callback1("hello", get-string-length).show)

  section("opaque pointers")
  val h = rust-handle(3)
  println("Created handle from rust: " ++ h.show)
  println("its address is " ++ get-memory-address(h.internal).show)
  println("Showing it again to validate refcounting: " ++ h.show)
  consume(h)
