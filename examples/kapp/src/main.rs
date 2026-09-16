// SPDX-FileCopyrightText: 2026 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use cxx_kde_frameworks::kconfigwidgets::kstylemanager;
use cxx_kde_frameworks::kcoreaddons::{KAboutData, KAboutPerson, License};
use cxx_kde_frameworks::kcrash::KCrash;
use cxx_kde_frameworks::ki18n::{self, KLocalizedString, i18nc};
use cxx_kde_frameworks::kiconthemes::KIconTheme;
use cxx_qt_lib::{QGuiApplication, QQuickStyle, QString, QUrl};
use qtbridge::QApp;

fn main() {
    let mut app = QApp::new();

    KIconTheme::init_theme();
    KCrash::initialize();

    KLocalizedString::set_application_domain("kapp");
    QGuiApplication::set_desktop_file_name(&QString::from("org.kde.kapp"));

    // To ensure the style is set correctly
    kstylemanager::init_style(); // OR do it manually as below
    if std::env::var("QT_QUICK_CONTROLS_STYLE").is_err() {
        QQuickStyle::set_style(&QString::from("org.kde.desktop"));
    }

    let mut about_data = KAboutData::from(
        "kapp",
        "KApp",
        "1.0",
        "Sample Rust Application!",
        License::GPL_V3,
    );

    if let Some(about) = about_data.as_mut() {
        about
            .add_author(&KAboutPerson::from(
                "Konqi",
                "Conqueror",
                "konqi@kde.org",
                "kde.org",
                &QUrl::from("https://konqi.png"),
            ))
            .set_translator(
                &i18nc("NAME OF TRANSLATORS", "Your names"),
                &i18nc("EMAIL OF TRANSLATORS", "Your emails"),
            );
    }

    KAboutData::set_application_data(&about_data);
    ki18n::setup_localized_context(&mut app);

    app.load_qml(include_bytes!("qml/Main.qml"));
    app.run();
}
