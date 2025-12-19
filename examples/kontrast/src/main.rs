// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use cxx_kde_frameworks::kcoreaddons::{KAboutData, KAboutPerson, License};
use cxx_kde_frameworks::kcrash::KCrash;
use cxx_kde_frameworks::ki18n::{i18nc, KLocalizedContext, KLocalizedString};
use cxx_qt::casting::Upcast;
use cxx_qt_lib::{QByteArray, QGuiApplication, QQmlApplicationEngine, QString, QUrl};

mod dbus;
mod kontrast;

#[tokio::main]
async fn main() {
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    KLocalizedString::set_application_domain(&QByteArray::from("kontrast"));
    KCrash::initialize();
    QGuiApplication::set_desktop_file_name(&QString::from("org.kde.kontrast"));

    let mut about_data = KAboutData::from(
        QString::from("konstrast"),
        i18nc("@title", "Kontrast"),
        QString::from("TEST"),
        i18nc("@title", "A constrast checker application. Now oxidized!"),
        License::GPL_V3,
    );

    if let Some(about) = about_data.as_mut() {
        about
            .add_author(&KAboutPerson::from(
                &i18nc("@info:credit", "Carl Schwan"),
                &i18nc("@info:credit", "Maintainer and creator"),
                &QString::from("carl@carlschwan.eu"),
                &QString::from("https://carlschwan.eu"),
                &QUrl::from("https://carlschwan.eu/avatar.png"),
            ))
            .add_credit(&KAboutPerson::from(
                &i18nc("@info:credit", "Wikipedia"),
                &i18nc("@info:credit", "Text on the main page CC-BY-SA-4.0"),
                &QString::default(),
                &QString::default(),
                &QUrl::default(),
            ))
            .add_author(&KAboutPerson::from(
                &i18nc("@info:credit", "Carson Black"),
                &i18nc("@info:credit", "SQLite backend for favorite colors"),
                &QString::default(),
                &QString::default(),
                &QUrl::default(),
            ))
            .set_translator(
                &i18nc("NAME OF TRANSLATORS", "Your names"),
                &i18nc("EMAIL OF TRANSLATORS", "Your emails"),
            );
    }

    KAboutData::set_application_data(&about_data);

    if let Some(mut engine) = engine.as_mut() {
        KLocalizedContext::initialize_engine(engine.as_mut().upcast_pin());
        engine.load(&QUrl::from("qrc:/qt/qml/org/kde/kontrast/src/qml/Main.qml"));
    }

    // Start the app
    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
