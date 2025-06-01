// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

#![doc = include_str!("../README.md")]

/// KCoreAddons provides classes built on top of QtCore to perform various tasks
/// [C++ API](https://api.kde.org/frameworks/kcoreaddons/html/namespaceKCoreAddons.html)
pub mod kcoreaddons;

/// KCrash provides support for intercepting and handling application crashes.
/// [C++ API](https://api.kde.org/frameworks/kcrash/html/namespaceKCrash.html)
pub mod kcrash;

/// KI18n provides functionality for internationalizing user interface text in applications, based on the GNU Gettext translation system.
/// It provides functionality to use translated strings from the Rust and QML interfaces.
/// [C++ API](https://api.kde.org/frameworks/ki18n/html/index.html)
pub mod ki18n;

/// This library contains classes to improve the handling of icons in applications using the KDE Frameworks.
/// [C++ API](https://api.kde.org/frameworks/kiconthemes/html/index.html)
pub mod kiconthemes;

/// KConfigWidgets provides easy-to-use classes to create configuration dialogs, as well as a set of widgets which uses KConfig to store their settings.
///
/// [C++ API documentation](https://api-staging.kde.org/kconfigwidgets-module.html)
pub mod kconfigwidgets;
