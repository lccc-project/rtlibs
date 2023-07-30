#![no_std]
#![no_builtins]
#![allow(incomplete_features)]
#![feature(generic_const_exprs, cfg_target_has_atomic)]

mod align;
mod os_prims;

mod arith;

#[repr(usize)]
pub enum Ordering {
    Relaxed = 0,
    Acquire = 1,
    Release = 2,
    AcqRel = 3,
    SeqCst = 4,
}

pub mod arch;
pub mod locking;

pub mod export;
