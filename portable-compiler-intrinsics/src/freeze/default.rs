pub unsafe fn freeze<T>(x: *mut T) {
    x.write_volatile(x.read_volatile());
}
