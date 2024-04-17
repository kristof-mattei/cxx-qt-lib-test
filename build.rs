use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    // if mac
    // .qt_module("Network")
    CxxQtBuilder::new()
        .qml_module(QmlModule {
            uri: "cxx_qt_lib_test",
            rust_files: &["src/cxxqt_object.rs"],
            qml_files: &["qml/main.qml"],
            ..Default::default()
        })
        .cc_builder(|cc| {
            let path = "init.cpp";
            cc.file(path);

            println!("cargo:rerun-if-changed={}", path);
        })
        .build();

    // this does not help
    println!("cargo:rustc-link-lib=cxx-qt-generated");
}
