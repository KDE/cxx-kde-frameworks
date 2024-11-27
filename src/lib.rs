// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

#![doc = include_str!("../README.md")]
pub mod kcoreaddons;
pub mod kcrash;

/// KI18n provides functionality for internationalizing user interface text in applications, based on the GNU Gettext translation system.
/// It provides functionality to use translated strings from the Rust and QML interfaces.
///
/// [C++ API](https://api.kde.org/frameworks/ki18n/html/index.html)
pub mod ki18n;
pub mod kiconthemes;
