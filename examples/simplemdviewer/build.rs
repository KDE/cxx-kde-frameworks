// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .qt_module("Network")
        .qml_module(QmlModule {
            uri: "org.kde.simplemdviewer",
            rust_files: &["src/bridge.rs"],
            qml_files: &["src/qml/main.qml"],
            ..Default::default()
        })
        .build();
}
