use core::cell::UnsafeCell;

use crate::Ordering;

use portable_compiler_intrinsics::{freeze_bytes, transmute_unchecked};

use crate::align::{self, NaturalAlignment};

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[cfg(any())]
mod x86;

#[cfg(any(target_arch = "clever", all()))]
mod clever;

pub unsafe trait ArchAtomicFlag: Copy {
    type Underlying: Copy + NaturalAlignment;

    unsafe fn test_and_set(p: *mut Self, ord: Ordering) -> Self;
    unsafe fn clear(p: *mut Self, ord: Ordering);
}

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

    unsafe fn swap(p: *mut Self, val: Self, ord: Ordering) -> Self;
}

pub unsafe trait ArchAtomicOps: ArchAtomic {
    unsafe fn fetch_add(p: *mut Self, val: Self, ord: Ordering) -> Self;
    unsafe fn fetch_sub(p: *mut Self, val: Self, ord: Ordering) -> Self;
}

#[repr(C)]
pub struct AtomicFlag(ArchAtomicStorage<<bool as ArchAtomicFlag>::Underlying>);

impl AtomicFlag {
    pub const fn new(x: bool) -> Self {
        Self(ArchAtomicStorage::new(x as _))
    }

    pub fn into_inner(self) -> bool {
        (self.0.into_inner() as usize) != 0
    }

    pub fn test_and_set(&self, ord: Ordering) -> bool {
        unsafe { ArchAtomicFlag::test_and_set(self.0.get().cast::<bool>(), ord) }
    }
    pub fn clear(&self, ord: Ordering) {
        unsafe { ArchAtomicFlag::clear(self.0.get().cast::<bool>(), ord) }
    }
}

#[repr(C)]
pub struct ArchAtomicStorage<T: NaturalAlignment>(UnsafeCell<T>, <T as NaturalAlignment>::Align);

unsafe impl<T: NaturalAlignment + Send> Sync for ArchAtomicStorage<T> {}

impl<T: NaturalAlignment> ArchAtomicStorage<T> {
    pub const fn new(x: T) -> Self {
        Self(UnsafeCell::new(x), align::new_alignment())
    }
    pub fn into_inner(self) -> T {
        self.0.into_inner()
    }

    pub const fn get(&self) -> *mut T {
        self.0.get()
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.0.get_mut()
    }
}

#[repr(C)]
pub struct AtomicArchImpl<T: NaturalAlignment>(ArchAtomicStorage<T>)
where
    [u8; core::mem::size_of::<T>()]: ArchAtomic;

impl<T: NaturalAlignment> AtomicArchImpl<T>
where
    [u8; core::mem::size_of::<T>()]: ArchAtomic,
{
    pub const fn new(x: T) -> Self {
        Self(ArchAtomicStorage::new(x))
    }

    pub fn into_inner(self) -> T {
        self.0.into_inner()
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.0.get_mut()
    }

    pub fn load(&self, order: Ordering) -> T {
        // SAFETY:
        // `self.0.get()` is valid, as it is derived from a safe reference to an `UnsafeCell`
        unsafe {
            transmute_unchecked(ArchAtomic::load(
                self.0.get().cast::<[u8; core::mem::size_of::<T>()]>(),
                order,
            ))
        }
    }

    pub fn store(&self, val: T, order: Ordering) {
        let bytes = freeze_bytes(val);

        unsafe {
            ArchAtomic::store(self.0.get().cast(), bytes, order);
        }
    }

    pub fn compare_exchange(
        &self,
        expected: T,
        new: T,
        success_order: Ordering,
        fail_order: Ordering,
    ) -> Result<T, T> {
        let expected_bytes = freeze_bytes(expected);
        let new_bytes = freeze_bytes(new);

        let (res_bytes, succ) = unsafe {
            ArchAtomic::compare_exchange(
                self.0.get().cast(),
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
    ) -> Result<T, T> {
        let expected_bytes = freeze_bytes(expected);
        let new_bytes = freeze_bytes(new);

        let (res_bytes, succ) = unsafe {
            ArchAtomic::compare_exchange_weak(
                self.0.get().cast(),
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
