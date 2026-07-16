// SPDX-FileCopyrightText: 2026 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use cxx_kde_frameworks::kconfigwidgets::kstylemanager;
use cxx_kde_frameworks::kcrash::KCrash;
use cxx_kde_frameworks::ki18n::{self, KLocalizedString};
use cxx_kde_frameworks::kiconthemes::KIconTheme;
use qtbridge::QApp;

fn main() {
    let mut app = QApp::new();
    
    KIconTheme::init_theme();
    KCrash::initialize();
    kstylemanager::init_style();
    
    KLocalizedString::set_application_domain("kapp");
    ki18n::setup_localized_context(&mut app);

    app.load_qml(include_bytes!("qml/Main.qml"));
    app.run();
}
