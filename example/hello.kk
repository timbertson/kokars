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

// for documentation purposes - no type safety here
alias raw-handle = any

struct rust-handle {
  internal: raw-handle
}

extern kkrs-generate-handle(i: int32): rust-handle
  c "kk_generate_handle"

extern raw-increment-id(h: raw-handle): raw-handle
  c "kk_handle_increment_id"
  
fun increment-id(h: rust-handle)
  match h
    Rust-handle(inner) -> Rust-handle(raw-increment-id(inner))

extern rust-handle/show(^h: rust-handle): string
  c "kk_show_handle"

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
  val h = kkrs-generate-handle(3.int32)
  println("Created handle from rust:")
  println(h.show)
  println("Showing it again to validate refcounting:")
  println(h.show)
  consume(h)


  section("uniqueness optimisation")
  val h1 = kkrs-generate-handle(5.int32)
  val h2 = h1.increment-id()
  val h3 = h2.increment-id()
  println("h2 is: " ++ h2.show)
  println("h3 is: " ++ h3.show)
