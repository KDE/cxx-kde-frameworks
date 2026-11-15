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

/// Class to use/access icon themes in KDE.
///
/// [C++ API documentation](https://api.kde.org/kicontheme.html)
pub use ffi::KIconTheme;

impl ffi::KIconTheme {
    /// Enforces the Breeze icon theme (including our KIconEngine for re-coloring).
    ///
    /// [C++ API documentation](https://api.kde.org/kicontheme.html#initTheme)
    pub fn init_theme() {
        ffi::init_icons();
    }
}
