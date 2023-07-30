#![no_std]
#![no_builtins]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod freeze;

pub use freeze::freeze;

mod transmute;

pub use transmute::transmute_unchecked;

pub fn freeze_bytes<T>(x: T) -> [u8; core::mem::size_of::<T>()]
where
    [u8; core::mem::size_of::<T>()]:,
{
    unsafe { freeze(transmute_unchecked(x)).assume_init() }
}

mod memcmp;

pub use memcmp::compare_bytes;

mod abort;
