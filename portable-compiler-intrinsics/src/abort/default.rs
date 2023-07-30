pub fn abort() -> ! {
    struct Guard;
    impl Drop for Guard {
        fn drop(&mut self) {
            panic!()
        }
    }

    let g = Guard;
    panic!()
}
