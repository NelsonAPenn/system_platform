# system\_platform

Syscalls in inline assembly from Rust (instead of using libc)-- just for fun.

This could be theoretically used to write `#![no_std]` programs that run on Linux.

I started with aarch64, but more architectures could be added.

Given infinite time and a better source of documentation, this could become an implementation of a new Rust standard library target, e.g. `aarch64-linux-unknown`, `aarch64-linux-none`, or similar.

## Usefulness or lack thereof

This project is only for fun. If you want the benefits of this library with stability and support, statically link musl.

I will say though, that if a similar implementation were created for MacOS (perhaps some or even most of these syscalls are the same given shared Unix heritage), it could solve the horrendous headache of cross-compilation for a Mac, which can only currently be done by jumping through licensing hoops and coping the MacOS SDK out of Xcode... yada yada.
