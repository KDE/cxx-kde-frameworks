// SPDX-FileCopyrightText: 2024 Jonah Brüchert <jbb@kaidan.im>
// SPDX-License-Identifier: MPL-2.0

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-kde-frameworks/kiconthemes/kicontheme.h");
        type KIconTheme;
    }

    #[namespace = "rust::bridge::kiconthemes"]
    unsafe extern "C++" {
        #[rust_name = "init_theme"]
        fn initTheme();
    }
}

/// Class to use/access icon themes in KDE.
///
/// [C++ API documentation](https://api.kde.org/kicontheme.html)
pub use ffi::KIconTheme;

impl ffi::KIconTheme {
    /// Enforces the Breeze icon theme (including our KIconEngine for re-coloring).
    ///
    /// [C++ API documentation](https://api.kde.org/kicontheme.html#initTheme)
    pub fn init_theme() {
        ffi::init_theme();
    }
}
