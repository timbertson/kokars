extern import
  c { library="kokars_examples" }

extern import
  c file "rs/generated/examples.h"

extern hello-kkrs(): io-noexn ()
  c "hello_kkrs"

fun main()
  hello-kkrs()
