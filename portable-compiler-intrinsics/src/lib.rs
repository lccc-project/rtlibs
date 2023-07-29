#![no_std]
#![no_builtins]
#![feature(generic_const_exprs)]

mod freeze;

pub use freeze::freeze;

mod transmute;

pub use transmute::transmute_unchecked;

mod memcmp;

pub use memcmp::compare_bytes;
