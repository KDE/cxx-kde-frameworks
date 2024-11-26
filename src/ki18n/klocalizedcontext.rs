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
        #[rust_name = "initialize_engine"]
        fn initializeEngine(engine: Pin<&mut QQmlEngine>);
    }
}
use core::pin::Pin;
use cxx_qt_lib::QQmlEngine;

pub use ffi::KLocalizedContext;

impl KLocalizedContext {
    pub fn initialize_engine(engine: Pin<&mut QQmlEngine>) {
        ffi::initialize_engine(engine);
    }
}
