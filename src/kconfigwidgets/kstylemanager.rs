// SPDX-FileCopyrightText: 2024 Jonah Brüchert <jbb@kaidan.im>
// SPDX-License-Identifier: MPL-2.0

#[cxx_qt::bridge]
mod ffi {
    #[namespace = "KStyleManager"]
    unsafe extern "C++" {
        include!("cxx-kde-frameworks/kstylemanager.h");

        /// Enforces the style configured by the user with fallback to the Breeze style.
        ///
        /// [C++ API documentation](https://api.kde.org/kstylemanager.html#initStyle)
        #[rust_name = "init_style"]
        fn initStyle();
    }
}

pub use ffi::init_style;
