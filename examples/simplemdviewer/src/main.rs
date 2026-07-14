// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use qtbridge::{QApp, QmlRegister, qobject};
// use std::env;

#[derive(Default)]
pub struct Converter;

#[qobject(NoQmlElement)]
impl Converter {
    #[qslot(qml_name = "mdFormat")]
    fn md_format(&mut self, text: String) -> String {
        markdown::to_html(text.as_str())
    }
}

impl QmlRegister for Converter {
    const URI: &str = "org.kde.simplemdviewer";
    const ELEMENT_NAME: &str = "MdConverter";
    const MINOR_VERSION: u8 = 1u8;
    const MAJOR_VERSION: u8 = 0u8;
    const IS_SINGLETON: bool = false;
}

fn main() {
    let mut app = QApp::new();
    app.register::<Converter>();

    // // To associate the executable to the installed desktop file
    // QGuiApplication::set_desktop_file_name(&QString::from("org.kde.simplemdviewer"));

    // // To ensure the style is set correctly
    // if env::var("QT_QUICK_CONTROLS_STYLE").is_err() {
    //     QQuickStyle::set_style(&QString::from("org.kde.desktop"));
    // }
    
    app.load_qml(include_bytes!("qml/Main.qml"));
    app.run();
}
