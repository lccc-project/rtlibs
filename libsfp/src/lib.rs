#![no_std]
#![no_builtins]
#![cfg_attr(enable_fenv, lcrust::fenv_mode(undefined))]

pub mod fp;
pub mod traits;
