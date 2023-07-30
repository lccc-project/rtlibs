extern "rust-intrinsic" {
    fn __builtin_abort() -> !;
}

pub fn abort() -> ! {
    __builtin_abort()
}
