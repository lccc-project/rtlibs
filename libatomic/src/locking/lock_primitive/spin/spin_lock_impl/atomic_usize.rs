use core::sync::atomic::{AtomicUsize, Ordering};

#[repr(transparent)]
pub struct SpinLockImpl(AtomicUsize);

const LOCK_ENGAGED: usize = 0x01;

impl SpinLockImpl {
    pub const fn new() -> Self {
        Self(AtomicUsize::new(0))
    }

    pub fn try_lock(&self) -> bool {
        self.0
            .compare_exchange(0, LOCK_ENGAGED, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }

    pub fn lock(&self) {
        let mut step_counter = 0usize;
        while self
            .0
            .compare_exchange_weak(0, LOCK_ENGAGED, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            if step_counter >= 16 {
                crate::os_prims::yield_thread();
            } else {
                core::hint::spin_loop();
            }

            step_counter += 1;
        }
    }

    pub unsafe fn unlock(&self) {
        self.0.store(0, Ordering::Release);
    }
}
