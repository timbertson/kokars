# kokars

`kokars` is an experiment in implementing native koka functionality in rust, rather than C.

# What is the plan?

It is an experiment, it is awkward, and it may never go anywhere. It's just an interesting challenge for now.

Specifically, I think this would be a terrible idea to rely on. I think one day the koka compiler _could_ export rust definitions for structs etc, and then that would be quite viable. But I don't think that's a useful thing for the koka compiler to focus on, there are much more useful things to work on.

And even if it can be integrated, the whole build toolchain story is likely to be a mess. This code uses `cargo` to build, which is way too much baggage to add to koka proper.

# How much overhead does it add?

I haven't really looked into it. The binaries are bigger, which isn't surprising. I think the runtime overheads should be equivalent to inlined C code. There may be some places where kokars has to use functions in order to wrap C macros might prevent some optimisations (but llvm is amazing, so maybe not).

# What are the downsides?

The main one is that structs defined in koka can't be shared with rust code, you have to redefine them in rust and be very careful to match all the fields exactly or else you'll be hosting a memory corruption party. There might be some things the koka compiler does for some struct definitions which don't match the obvious thing in rust, so maybe even being careful isn't enough.

Also, it's just really tedious to define all the types (again, carefully) for the various kklib C APIs you want to use in rust.

It's possible that `rust-bindgen` could address both of these, but I haven't looked at how to integrate it.

# What are the upsides?

It's Rust! Safety! Good tooling, and all around good vibes!

Honestly I don't know how much rust's safety would fortify real code, since you're doing a lot of C FFI and generally being careful to write types correctly whenever you're crossing that boundary. But koka's memory management is very compatible with rust's `Clone` and `Drop`, so the compiler will enforce that you are (under the hood) calling `dup` and `drop` at all the right times. That's the coolest part, I think.

# Information flow:

## Generation of rust structs for builtin types

kokars/bootstrap/generate.kk generates rust source code defining rust structs with the same size as various C structs. This doesn't let us access the fields, but we can pass them (by value) because the sizes match.
It also generates `Drop` and `Clone` implementations calling the relevant `_dup` and `_drop` functions.

These definitions (specifically the sizes) are not stable across systems, koka versions and possibly even different compiler flags.

## rust kokars crate

This crate is built by cargo, depending on only the generated code above. It contains many `extern` function definitions which will need to be present at link time, provided by koka.

It also contains all the convenience rust APIs which map onto the low-level C ones. E.g. to get a &str from a &kk_string_t (with an accurate lifetime).

It's a `no_std` crate, but exposes a `global_allocator` interface to koka's memory management. So `Box` and other rust features requiring allocation should work.

## kokars.kk

This module is mostly a C extension to add additional symbols needed by the rust `kklib` crate - i.e. things that logically belong in upstream koka, but aren't (for various reasons, often inlined functions or macros which rust can't link against).

# How it's used:

Your rust code depends on the `kklib` crate. Every function to be called from koka must be `#[unsafe(no_mangle)] pub extern "C" fn ...`

Then your koka code imports the `kokars` (koka) module, ensuring everything the `kokars` rust crate needs at link time will be present.

# Naming conventions

Lowercase types (e.g. `kk_string_t`) are the regular koka types. Titlecase types like `KkFunction` are rust wrappers, often adding additional type safetly or functionality not directly exposed in koka's C API.

Functions with `_c` appended are reexports of static functions - e.g. `kk_to_ssize_t_c` is a regular function (visible to the linker) which invokes the `kk_to_ssize_t` static C function (not visible to the linker).
