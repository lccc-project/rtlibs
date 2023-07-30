use core::cell::UnsafeCell;

use crate::align::{self, NaturalAlignment};

use crate::Ordering;

mod lock_primitive;

static __ATOMIC_LOCK: lock_primitive::LockImpl = lock_primitive::LockImpl::new();

#[repr(C)]
pub struct AtomicLockingImpl<T: NaturalAlignment>(UnsafeCell<T>, <T as NaturalAlignment>::Align);

unsafe impl<T: NaturalAlignment + Copy> Sync for AtomicLockingImpl<T> {}

impl<T: NaturalAlignment + Send + Copy> AtomicLockingImpl<T>
where
    [u8; core::mem::size_of::<T>()]:,
{
    pub const fn new(x: T) -> Self {
        Self(UnsafeCell::new(x), align::new_alignment())
    }

    pub fn store(&self, val: T, _: Ordering) {
        let _guard = __ATOMIC_LOCK.lock();
        let buf: [u8; core::mem::size_of::<T>()] = unsafe {
            portable_compiler_intrinsics::freeze(portable_compiler_intrinsics::transmute_unchecked(
                val,
            ))
            .assume_init()
        };
        unsafe {
            self.0
                .get()
                .copy_from_nonoverlapping(&buf as *const u8 as *const T, 1)
        }
    }

    pub fn load(&self, _: Ordering) -> T {
        let _guard = __ATOMIC_LOCK.lock();
        unsafe { self.0.get().read() }
    }

    pub fn compare_exchange(&self, expected: T, new: T, _: Ordering, _: Ordering) -> Result<T, T> {
        let buf: [u8; core::mem::size_of::<T>()] = unsafe {
            portable_compiler_intrinsics::freeze(portable_compiler_intrinsics::transmute_unchecked(
                expected,
            ))
            .assume_init()
        };
        let _guard = __ATOMIC_LOCK.lock();

        let vbuf: [u8; core::mem::size_of::<T>()] = unsafe {
            portable_compiler_intrinsics::freeze(portable_compiler_intrinsics::transmute_unchecked(
                self.0.get().read(),
            ))
            .assume_init()
        };

        if buf == vbuf {
            let val: [u8; core::mem::size_of::<T>()] = unsafe {
                portable_compiler_intrinsics::freeze(
                    portable_compiler_intrinsics::transmute_unchecked(new),
                )
                .assume_init()
            };

            unsafe {
                self.0
                    .get()
                    .copy_from_nonoverlapping(&buf as *const u8 as *const T, 1)
            }

            Ok(unsafe { portable_compiler_intrinsics::transmute_unchecked(vbuf) })
        } else {
            Err(unsafe { portable_compiler_intrinsics::transmute_unchecked(vbuf) })
        }
    }

    pub fn compare_exchange_weak(
        &self,
        expected: T,
        new: T,
        success_order: Ordering,
        fail_order: Ordering,
    ) -> Result<T, T> {
        self.compare_exchange(expected, new, success_order, fail_order)
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.0.get_mut()
    }
}
