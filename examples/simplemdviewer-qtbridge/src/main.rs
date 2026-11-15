// SPDX-FileCopyrightText: 2026 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use cxx_qt_lib::{QGuiApplication, QQuickStyle};
use qtbridge::{QApp, QmlElement, qobject};

#[derive(Default)]
pub struct Converter;

#[qobject(NoQmlElement)]
impl Converter {
    #[qslot(qml_name = "mdFormat")]
    fn md_format(&mut self, text: String) -> String {
        markdown::to_html(text.as_str())
    }
}

impl QmlElement for Converter {
    const URI: &str = "org.kde.simplemdviewer";
    const ELEMENT_NAME: &str = "MdConverter";
    const MINOR_VERSION: u8 = 1u8;
    const MAJOR_VERSION: u8 = 0u8;
    const IS_SINGLETON: bool = false;
}

fn main() {
    let mut app = QApp::new();
    app.register::<Converter>();

    // To associate the executable to the installed desktop file
    QGuiApplication::set_desktop_file_name(&"org.kde.simplemdviewer".into());

    // To ensure the style is set correctly
    if std::env::var("QT_QUICK_CONTROLS_STYLE").is_err() {
        QQuickStyle::set_style(&"org.kde.desktop".into());
    }

    app.load_qml(include_bytes!("qml/Main.qml"));
    app.run();
}
