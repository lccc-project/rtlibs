pub fn abort() -> ! {
    unsafe { core::arch::asm!("ud2", options(noreturn)) }
}
