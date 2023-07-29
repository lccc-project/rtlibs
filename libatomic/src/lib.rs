#![no_std]
#![no_builtins]
#![feature(generic_const_exprs, cfg_target_has_atomic)]

mod align;
mod os_prims;

pub mod arch;
pub mod locking;
