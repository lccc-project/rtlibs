use crate::align::{self, NaturalAlignment};

use self::spin_lock_impl::SpinLockImpl;

mod spin_lock_impl {
    use crate::Ordering;

    use crate::arch::AtomicFlag;

    #[repr(transparent)]
    pub struct SpinLockImpl(AtomicFlag);

    impl SpinLockImpl {
        pub const fn new() -> Self {
            Self(AtomicFlag::new(false))
        }

        pub fn try_lock(&self) -> bool {
            !self.0.test_and_set(Ordering::Acquire)
        }

        pub fn lock(&self) {
            let mut step_counter = 0usize;
            while self.0.test_and_set(Ordering::Acquire) {
                if step_counter >= 16 {
                    crate::os_prims::yield_thread();
                } else {
                    core::hint::spin_loop();
                }

                step_counter += 1;
            }
        }

        pub unsafe fn unlock(&self) {
            self.0.clear(Ordering::Release);
        }
    }
}

#[repr(C)]
pub struct SpinLock(SpinLockImpl, <usize as NaturalAlignment>::Align);

impl SpinLock {
    pub const fn new() -> Self {
        Self(SpinLockImpl::new(), align::new_alignment())
    }

    pub fn lock(&self) -> Guard<'_> {
        self.0.lock();
        Guard { lock: &self.0 }
    }

    pub fn try_lock(&self) -> Result<Guard<'_>, ()> {
        if self.0.try_lock() {
            Ok(Guard { lock: &self.0 })
        } else {
            Err(())
        }
    }

    pub fn force_unlock(&mut self) {
        unsafe { self.0.unlock() }
    }
}

pub struct Guard<'a> {
    lock: &'a SpinLockImpl,
}

impl<'a> Drop for Guard<'a> {
    fn drop(&mut self) {
        unsafe {
            self.lock.unlock();
        }
    }
}
