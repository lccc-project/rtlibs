#![no_std]
#![no_builtins]

mod freeze;

pub use freeze::freeze;

mod transmute;

pub use transmute::transmute_unchecked;
