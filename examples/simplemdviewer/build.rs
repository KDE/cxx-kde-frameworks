// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new_qml_module(
        QmlModule::new("org.kde.simplemdviewer").qml_file("src/qml/Main.qml"),
    )
    .files(["src/bridge.rs"])
    .qt_module("Network")
    .build();
}
