// SPDX-FileCopyrightText: 2026 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use qtbridge::QApp;
use qtbridge_type_lib::QQmlApplicationEngine;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-kde-frameworks/src/ki18n/klocalization.h");

        include!(
            "qtbridge-type-lib/src/generated/qml/qqmlapplicationengine/cpp/qqmlapplicationengine.h"
        );
        type QQmlApplicationEngine = super::QQmlApplicationEngine;
    }

    #[namespace = "rust::bridge::klocalization"]
    unsafe extern "C++" {
        # [rust_name = inline_cpp_fn_setup_localized_context]
        fn inlineCppFn_setupLocalizedContext(engine: Pin<&mut QQmlApplicationEngine>);
    }
}

///
/// [C++ API documentation](https://api.kde.org/klocalizedqmlcontext.html#setupLocalizedContext)
///
/// # Usage
/// ```no_run
///
/// use qtbridge::QApp;
/// use cxx_kde_frameworks::ki18n;
///
/// let mut app = QApp::new();
/// ki18n::setup_localized_context(&mut app);
/// ```

#[allow(dead_code)]
pub fn setup_localized_context(app: &mut QApp) {
    if let Some(engine) = app.engine.as_mut() {
        ffi::inline_cpp_fn_setup_localized_context(engine);
    }
}
