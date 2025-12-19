// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new_qml_module(QmlModule::new("org.kde.kontrast").qml_files([
        "src/qml/Main.qml",
        "src/qml/MainPage.qml",
        "src/qml/AboutPage.qml",
        "src/qml/FavoritePage.qml",
        "src/qml/HelpPage.qml",
    ]))
    .file("src/kontrast.rs")
    .qrc("src/resources.qrc")
    .build();
}
