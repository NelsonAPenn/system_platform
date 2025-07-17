# system\_platform

Linux syscalls in inline assembly from Rust (instead of using libc)-- just for fun.

This is primarily a learning exercise for me, I recently discovered a bunch of
other crates that have already done this.

Current small feature set is supported on ARM64, ARM32, and x86_64.

## Features

- a syscall! macro which factors out the syscall calling conventions for the
  major architectures.
- light wrapping functions for the syscalls above at the would-be-libc level of
  abstraction

## Interesting effects of this approach

1. This could be theoretically used to write `#![no_std]` programs that run on
   Linux.
  - This is in progress but not quite working, but main can be built with target
    x86_64-unknown-none and `#![no_std]`, it just segfaults.
  - I believe this is likely the entrypoint still not being correct. And maybe
    needing to "set up the stack".
2. As a result of 1, these executables would not depend on libc and might be a
   little more portable. From what I understand, Go uses this approach.

## Future directions

Given infinite time and a better source of documentation, this could become an
implementation of a new Rust standard library target (most likely resulting in
the triples `aarch64-linux-none`, `x86_64-linux-none`, etc.).

## Usefulness or lack thereof

This project is only for fun. If you want the benefits of this library with
stability and support, statically link musl.

I will say though, that if a similar implementation were created for MacOS
(perhaps some or even most of these syscalls are the same given shared Unix
heritage), it could solve the horrendous headache of cross-compilation for a
Mac, which can only currently be done by jumping through licensing hoops and
copying the MacOS SDK out of Xcode... yada yada. However, I looked into this a
little more and Apple claims that the syscall interface is unstable and to use
all the MacOS SDK crap. Big suprise there. But should we listen?
