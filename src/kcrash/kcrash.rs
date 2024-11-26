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

pub struct KCrash {}

impl KCrash {
    pub fn initialize() {
        ffi::initialize_kcrash();
    }
}
