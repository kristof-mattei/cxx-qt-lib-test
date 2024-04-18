use cxx_qt_lib_test::init_resources;

#[test]
fn init() {
    init_resources();
}

#[test]
fn assert_world_ok() {
    let cls1 = || true;
    let cls2 = || true;
    assert_eq!(cls1(), cls2());
}
