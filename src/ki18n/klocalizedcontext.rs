// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/qqmlengine.h");
        type QQmlEngine = cxx_qt_lib::QQmlEngine;

        include!("cxx-kde-frameworks/klocalizedcontext.h");

        #[qobject]
        type KLocalizedContext;
    }

    #[namespace = "rust::kf6"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "initialize_engine"]
        fn initializeEngine(engine: Pin<&mut QQmlEngine>);
    }
}
use core::pin::Pin;
use cxx_qt_lib::QQmlEngine;

/// Simplifies integration of [KI18n](https://api.kde.org/frameworks/ki18n/html/index.html) framework in QML.
///
/// [C++ API](https://api.kde.org/frameworks/ki18n/html/classKLocalizedContext.html)
///
/// # Usage
/// ```no_run
/// use cxx_qt_lib::QQmlApplicationEngine;
/// use cxx_kde_frameworks::ki18n::KLocalizedContext;
///
/// let mut engine = QQmlApplicationEngine::new();
/// if let Some(mut engine) = engine.as_mut() {
///     KLocalizedContext::initialize_engine(engine.as_mut().as_qqmlengine());
/// }
/// ```
pub use ffi::KLocalizedContext;

impl KLocalizedContext {
    /// Initializes KLocalizedContext inside QQmlEngine by setting it's root context to a new instance of KLocalizedContext.
    pub fn initialize_engine(engine: Pin<&mut QQmlEngine>) {
        ffi::initialize_engine(engine);
    }
}
