use core::mem::MaybeUninit;

pub unsafe trait AtomicWithArith: Sized {
    const IS_SIGNED: bool;
}

unsafe impl<T: AtomicWithArith> AtomicWithArith for MaybeUninit<T> {
    const IS_SIGNED: bool = T::IS_SIGNED;
}

macro_rules! why_am_i_using_a_macro_again{
    ($($uty:ty),*; $($sty:ty),*;) => {
        $(
            unsafe impl AtomicWithArith for $uty{
                const IS_SIGNED: bool = false;
            }
        )*

        $(
            unsafe impl AtomicWithArith for $sty{
                const IS_SIGNED: bool = true;
            }
        )*
    }
}
why_am_i_using_a_macro_again! {
    u8, u16, u32, u64, u128, usize;
    i8, i16, i32, i64, i128, isize;
}
