// SPDX-FileCopyrightText: 2024 Jonah Brüchert <jbb@kaidan.im>
// SPDX-License-Identifier: MPL-2.0

#[cxx_qt::bridge]
mod ffi {
    #[namespace = "rust::kf6"]
    unsafe extern "C++" {
        include!("cxx-kde-frameworks/kcrash.h");

        #[rust_name = "initialize_kcrash"]
        fn initializeKCrash();
    }
}

/// Functions to handle crashes
///
/// [C++ API documentation](https://api.kde.org/kcrash.html)
pub struct KCrash {}

impl KCrash {

    /// Initialize KCrash.
    ///
    /// [C++ API documentation](https://api.kde.org/kcrash.html#initialize)
    pub fn initialize() {
        ffi::initialize_kcrash();
    }
}
