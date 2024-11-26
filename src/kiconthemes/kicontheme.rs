// SPDX-FileCopyrightText: 2024 Jonah Brüchert <jbb@kaidan.im>
// SPDX-License-Identifier: MPL-2.0

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-kde-frameworks/kicontheme.h");
        type KIconTheme;
    }

    #[namespace = "rust::kf6"]
    unsafe extern "C++" {
        #[rust_name = "init_icons"]
        fn initIcons();
    }
}

pub use ffi::KIconTheme;

impl ffi::KIconTheme {
    pub fn init_theme() {
        ffi::init_icons();
    }
}
