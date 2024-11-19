// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

#![doc = include_str!("../README.md")]

/// KCoreAddons provides classes built on top of QtCore to perform various tasks.
///
/// [C++ API documentation](https://api-staging.kde.org/kcoreaddons-module.html)
pub mod kcoreaddons;

/// KCrash provides support for intercepting and handling application crashes.
///
/// [C++ API documentation](https://api-staging.kde.org/kcrash-module.html)
pub mod kcrash;

/// KI18n provides functionality for internationalizing user interface text in applications, based on the GNU Gettext translation system.
/// It provides functionality to use translated strings from the Rust and QML interfaces.
///
/// [C++ API documentation](https://api-staging.kde.org/ki18n-module.html)
pub mod ki18n;

/// This library contains classes to improve the handling of icons in applications using the KDE Frameworks.
///
/// [C++ API documentation](https://api-staging.kde.org/kiconthemes-module.html)
pub mod kiconthemes;

/// KConfigWidgets provides easy-to-use classes to create configuration dialogs, as well as a set of widgets which uses KConfig to store their settings.
///
/// [C++ API documentation](https://api-staging.kde.org/kconfigwidgets-module.html)
pub mod kconfigwidgets;

/// KCMUtils is a collection of convenience classes and widgets to create config modules.
///
/// [C++ API documentation](https://api-staging.kde.org/kcmutils-index.html)
pub mod kcmutils;

pub mod kio;
