use core::{cell::UnsafeCell, sync::atomic::Ordering};

use crate::align::{self, NaturalAlignment};

mod lock_primitive;

static __ATOMIC_LOCK: lock_primitive::LockImpl = lock_primitive::LockImpl::new();

#[repr(C)]
pub struct AtomicLockingImpl<T: NaturalAlignment + Copy>(
    UnsafeCell<T>,
    <T as NaturalAlignment>::Align,
);

unsafe impl<T: NaturalAlignment + Send + Copy> Sync for AtomicLockingImpl<T> {}

impl<T: NaturalAlignment + Copy> AtomicLockingImpl<T> {
    pub const fn new(x: T) -> Self {
        Self(UnsafeCell::new(x), align::new_alignment())
    }

    pub fn store(&self, val: T, _: Ordering) {
        let _guard = __ATOMIC_LOCK.lock();
        unsafe { self.0.get().write(val) }
    }

    pub fn load(&self, _: Ordering) -> T {
        let _guard = __ATOMIC_LOCK.lock();
        unsafe { self.0.get().read() }
    }

    pub fn compare_exchange(&self, expected: T, new: T, _: Ordering, _: Ordering) -> Result<T, T>
    where
        [u8; core::mem::size_of::<T>()]:,
    {
        let mut buf: [u8; core::mem::size_of::<T>()] = unsafe { core::mem::transmute(expected) };
        let _guard = __ATOMIC_LOCK.lock();
    }
}
