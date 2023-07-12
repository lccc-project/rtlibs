use crate::align::{self, NaturalAlignment};

use self::spin_lock_impl::SpinLockImpl;

#[cfg(target_has_atomic = "ptr")]
mod spin_lock_impl {
    mod atomic_usize;

    pub use atomic_usize::*;
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
