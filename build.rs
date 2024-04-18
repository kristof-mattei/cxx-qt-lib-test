use std::io::Write;
use std::{env, fs::OpenOptions};

use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    let modules = [QmlModule {
        uri: "cxx_qt_lib_test",
        rust_files: &["src/cxxqt_object.rs"],
        qml_files: &["qml/main.qml"],
        ..Default::default()
    }];

    let mut builder = CxxQtBuilder::new().cc_builder(|cc| {
        let start = r#"
#include <QtPlugin>

void _init_qt_resources() {
    std::fprintf(stderr, "Initializing plugins...\n");
            "#;

        let end = r#"
    std::fprintf(stderr, "Done initializing plugins...\n");
}

extern "C" {
    void init_qt_resources() {
        _init_qt_resources();
    }
}
            "#;

        let cpp_directory = format!(
            "{}/cxx-qt-gen/src/init_hack.cpp",
            env::var("OUT_DIR").unwrap()
        );

        std::fs::create_dir_all(&cpp_directory).expect("Couldn't create dir");

        let path = format!("{}/init_hack.cpp", cpp_directory);

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .create(true)
            .open(&path)
            .expect("Couldn't create file");

        writeln!(file, "{}", start).expect("Failed to write start");

        for m in &modules {
            writeln!(file, "    Q_IMPORT_PLUGIN({}_plugin)", m.uri).expect("Failed to write piece");
        }

        writeln!(file, "{}", end).expect("Failed to write end");

        cc.file(path);
    });

    for m in modules {
        builder = builder.qml_module(m);
    }
    // if mac
    // .qt_module("Network")

    builder.build();
}
