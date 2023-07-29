use core::{cell::UnsafeCell, sync::atomic::Ordering};

use crate::align::NaturalAlignment;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

pub unsafe trait ArchAtomic: Copy {
    unsafe fn load(p: *const Self, ord: Ordering) -> Self;
    unsafe fn store(p: *mut Self, val: Self, ord: Ordering);
    unsafe fn compare_exchange_weak(
        p: *mut Self,
        expected: Self,
        new: Self,
        success_order: Ordering,
        fail_order: Ordering,
    ) -> (Self, bool);

    unsafe fn compare_exchange(
        p: *mut Self,
        expected: Self,
        new: Self,
        success_order: Ordering,
        fail_order: Ordering,
    ) -> (Self, bool)
    where
        [u8; core::mem::size_of::<Self>()]:,
    {
        loop {
            let (val, succ) =
                Self::compare_exchange_weak(p, expected, new, success_order, fail_order);

            if succ
                || unsafe { portable_compiler_intrinsics::compare_bytes(&expected, &val).is_ne() }
            {
                return (val, succ);
            }
        }
    }
}

#[repr(C)]
pub struct AtomicArchImpl<T: NaturalAlignment>(UnsafeCell<T>, <T as NaturalAlignment>::Align)
where
    [u8; core::mem::size_of::<T>()]: ArchAtomic;
