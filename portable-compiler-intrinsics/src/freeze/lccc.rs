extern "rust-intrinsic" {
    fn __builtin_read_freeze<T>(val: *const T) -> T;
}

pub unsafe fn freeze<T>(val: *mut T) {
    core::ptr::write(val, __builtin_read_freeze(val))
}

pub unsafe fn read_freeze<T>(val: *const T) -> T {
    __builtin_read_freeze(val)
}
