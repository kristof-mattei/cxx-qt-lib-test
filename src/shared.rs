pub fn init_resources() {
    #[link(name = "cxx-qt-generated")]
    extern "C" {
        fn init_qt_resources();
    }

    unsafe {
        init_qt_resources();
    }
}
