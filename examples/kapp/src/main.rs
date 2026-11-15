// SPDX-FileCopyrightText: 2026 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use cxx_kde_frameworks::kconfigwidgets::kstylemanager;
use cxx_kde_frameworks::kcoreaddons::{KAboutData, KAboutPerson, License};
use cxx_kde_frameworks::kcrash::KCrash;
use cxx_kde_frameworks::ki18n::{self, KLocalizedString, i18nc};
use cxx_kde_frameworks::kiconthemes::KIconTheme;
use cxx_qt_lib::{QGuiApplication, QQuickStyle, QString};
use qtbridge::QApp;

fn main() {
    let mut app = QApp::new();

    KIconTheme::init_theme();
    KCrash::initialize();

    KLocalizedString::set_application_domain(&"kapp".into());
    QGuiApplication::set_desktop_file_name(&"org.kde.kapp".into());

    // To ensure the style is set correctly
    kstylemanager::init_style(); // OR do it manually as below
    if std::env::var("QT_QUICK_CONTROLS_STYLE").is_err() {
        QQuickStyle::set_style(&QString::from("org.kde.desktop"));
    }

    let mut about_data = KAboutData::from(
        "kapp".into(),
        "KApp".into(),
        "1.0".into(),
        "Sample Rust Application!".into(),
        License::GPL_V3,
    );

    if let Some(about) = about_data.as_mut() {
        about
            .add_author(&KAboutPerson::from(
                &"Konqi".into(),
                &"Conqueror".into(),
                &"konqi@kde.org".into(),
                &"kde.org".into(),
                &"https://konqi.png".into(),
            ))
            .set_translator(
                &i18nc("NAME OF TRANSLATORS", "Your names"),
                &i18nc("EMAIL OF TRANSLATORS", "Your emails"),
            );
    }

    KAboutData::set_application_data(&about_data);
    ki18n::setup_localized_context(app.qml_engine());

    app.load_qml(include_bytes!("qml/Main.qml"));
    app.run();
}
