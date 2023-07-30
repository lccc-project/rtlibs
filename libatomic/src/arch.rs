use core::{cell::UnsafeCell, sync::atomic::Ordering};

use portable_compiler_intrinsics::{freeze, freeze_bytes, transmute_unchecked};

use crate::align::{self, NaturalAlignment};

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
    ) -> (Self, bool);
}

#[repr(C)]
pub struct AtomicArchImpl<T: NaturalAlignment>(
    UnsafeCell<[u8; core::mem::size_of::<T>()]>,
    <T as NaturalAlignment>::Align,
)
where
    [u8; core::mem::size_of::<T>()]: ArchAtomic;

impl<T: NaturalAlignment> AtomicArchImpl<T>
where
    [u8; core::mem::size_of::<T>()]: ArchAtomic,
{
    pub fn new(x: T) -> Self {
        Self(UnsafeCell::new(freeze_bytes(x)), align::new_alignment())
    }

    pub fn into_inner(self) -> T {
        unsafe { transmute_unchecked(self.0.into_inner()) }
    }

    pub fn get_mut(&mut self) -> &mut T {
        unsafe { &mut *(self.0.get_mut() as *mut _ as *mut T) }
    }

    pub fn load(&self, order: Ordering) -> T {
        // SAFETY:
        // `self.0.get()` is valid, as it is derived from a safe reference to an `UnsafeCell`
        unsafe { transmute_unchecked(ArchAtomic::load(self.0.get(), order)) }
    }

    pub fn store(&self, val: T, order: Ordering) {
        let bytes = freeze_bytes(val);

        unsafe {
            ArchAtomic::store(self.0.get(), bytes, order);
        }
    }

    pub fn compare_exchange(
        &self,
        expected: T,
        new: T,
        success_order: Ordering,
        fail_order: Ordering,
    ) -> Result<Self, Self> {
        let expected_bytes = freeze_bytes(expected);
        let new_bytes = freeze_bytes(new);

        let (res_bytes, succ) = unsafe {
            ArchAtomic::compare_exchange(
                self.0.get(),
                expected_bytes,
                new_bytes,
                success_order,
                fail_order,
            )
        };

        let val = unsafe { transmute_unchecked(res_bytes) };

        if succ {
            Ok(val)
        } else {
            Err(val)
        }
    }

    pub fn compare_exchange_weak(
        &self,
        expected: T,
        new: T,
        success_order: Ordering,
        fail_order: Ordering,
    ) -> Result<Self, Self> {
        let expected_bytes = freeze_bytes(expected);
        let new_bytes = freeze_bytes(new);

        let (res_bytes, succ) = unsafe {
            ArchAtomic::compare_exchange_weak(
                self.0.get(),
                expected_bytes,
                new_bytes,
                success_order,
                fail_order,
            )
        };

        let val = unsafe { transmute_unchecked(res_bytes) };

        if succ {
            Ok(val)
        } else {
            Err(val)
        }
    }
}
