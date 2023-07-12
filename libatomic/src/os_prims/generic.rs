pub fn yield_thread() {
    // can't do much here, just spin_loop_hint
    core::hint::spin_loop()
}
