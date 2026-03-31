// SPDX-FileCopyrightText: 2026 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

#[cxx_qt::bridge]
mod ffi {
    extern "C++Qt" {
        include!("cxx-qt-lib/qqmlengine.h");
        type QQmlEngine = cxx_qt_lib::QQmlEngine;

        include!("cxx-kde-frameworks/klocalization.h");
    }

    #[namespace = "rust::kf6"]
    unsafe extern "C++Qt" {
        #[doc(hidden)]
        #[rust_name = "setup_localized_context"]
        fn setupLocalizedContext(engine: Pin<&mut QQmlEngine>);
    }
}

use core::pin::Pin;
use cxx_qt_lib::QQmlEngine;

///  Use KI18n framework in QML.
///
/// [C++ API documentation](https://api.kde.org/klocalizedcontext.html)
///
/// # Usage
/// ```no_run
/// use cxx_qt_lib::QQmlApplicationEngine;
/// use cxx_kde_frameworks::ki18n;
/// use cxx_qt::casting::Upcast;
///
/// let mut engine = QQmlApplicationEngine::new();
/// if let Some(mut engine) = engine.as_mut() {
///     ki18n::setup_localized_context(engine.as_mut().upcast_pin());
/// }
/// ```

pub fn setup_localized_context(engine: Pin<&mut QQmlEngine>) {
    ffi::setup_localized_context(engine);
}
