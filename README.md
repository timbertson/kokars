# kokars

`kokars` demonstrates how to implement native koka functionality in rust, rather than C.

It is an experiment, it is awkward, and it may never go anywhere. It's just an interesting challenge for now.

# Information flow:

koka standard types:

 - bootstrap/generate.kk generates rust source code to define the C structs & functions implemented in kklib
 - rust code (in examples) can reference these types freely
 - cbindgen runs to export structs and functions defined in rust code in C headers, which can then be imported by koka code that wishes to use rust functions

# Todo:

 - Something about platform awareness (kklib types may differ in size, for example)
 - Publish a crate?
