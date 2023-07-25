use core::mem::MaybeUninit;

mod compiler_impl {
    include!(concat!("freeze/", env!("FREEZE_IMPL"), ".rs"));
}

pub unsafe fn freeze<T: Copy + 'static>(mut x: MaybeUninit<T>) -> MaybeUninit<T> {
    compiler_impl::freeze(x.as_mut_ptr());
    x
}
