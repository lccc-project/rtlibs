mod compiler_impl {
    include!(concat!("abort/", env!("ABORT_IMPL"), ".rs"));
}
