pub mod cxxqt_object;

pub fn init_resources() {
    #[link(name = "cxx-qt-generated", kind = "static")]
    extern "C" {
        fn init_qt_resources();
    }

    unsafe {
        init_qt_resources();
    }
}
