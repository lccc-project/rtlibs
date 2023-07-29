pub unsafe fn freeze<T>(x: *mut T) {
    x.write_volatile(x.read_volatile());
}

pub unsafe fn read_freeze<T>(x: *const T) -> T {
    x.read_volatile()
}
