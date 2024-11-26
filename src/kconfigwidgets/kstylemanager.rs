// SPDX-FileCopyrightText: 2024 Jonah Brüchert <jbb@kaidan.im>
// SPDX-License-Identifier: MPL-2.0

#[cxx_qt::bridge]
mod ffi {
    #[namespace = "KStyleManager"]
    unsafe extern "C++" {
        include!("cxx-kde-frameworks/kstylemanager.h");

        #[rust_name = "init_style"]
        fn initStyle();
    }
}

pub use ffi::init_style;
