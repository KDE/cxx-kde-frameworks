// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

#[cxx_qt::bridge]
mod ffi {

    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;

        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "C++" {
        include!("cxx-kde-frameworks/klocalizedstring.h");
        type KLocalizedString;
    }

    #[namespace = "rust::kf6"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "set_application_domain"]
        fn setApplicationDomain(domain: &QByteArray);

        #[doc(hidden)]
        #[rust_name = "application_domain"]
        fn applicationDomain() -> QByteArray;

        #[doc(hidden)]
        #[rust_name = "ki18n"]
        fn r_ki18n(text: String) -> UniquePtr<KLocalizedString>;

        #[doc(hidden)]
        #[rust_name = "ki18nc"]
        fn r_ki18nc(context: String, text: String) -> UniquePtr<KLocalizedString>;

        #[doc(hidden)]
        #[rust_name = "ki18np"]
        fn r_ki18np(singular: String, plural: String) -> UniquePtr<KLocalizedString>;

        #[doc(hidden)]
        #[rust_name = "ki18ncp"]
        fn r_ki18ncp(
            context: String,
            singular: String,
            plural: String,
        ) -> UniquePtr<KLocalizedString>;

        #[doc(hidden)]
        #[rust_name = "ki18nd"]
        fn r_ki18nd(domain: String, text: String) -> UniquePtr<KLocalizedString>;

        #[doc(hidden)]
        #[rust_name = "ki18ndc"]
        fn r_ki18ndc(domain: String, context: String, text: String) -> UniquePtr<KLocalizedString>;

        #[doc(hidden)]
        #[rust_name = "ki18ndp"]
        fn r_ki18ndp(
            domain: String,
            singular: String,
            plural: String,
        ) -> UniquePtr<KLocalizedString>;

        #[doc(hidden)]
        #[rust_name = "ki18ndcp"]
        fn r_ki18ndcp(
            domain: String,
            context: String,
            singular: String,
            plural: String,
        ) -> UniquePtr<KLocalizedString>;

    }

    unsafe extern "C++" {

        /// Finalizes and creates a translated [QString](cxx_qt_lib::QString).
        ///
        /// C++ counterpart: [KLocalizedString::toString()](https://api-staging.kde.org/klocalizedstring.html#toString).
        #[rust_name = "to_qstring"]
        fn toString(self: &KLocalizedString) -> QString;
    }
}

use cxx::UniquePtr;
use cxx_qt_lib::QByteArray;

/// Class for producing and handling localized messages.
///
/// [C++ API documentation](https://api-staging.kde.org/klocalizedstring.html)
pub use ffi::KLocalizedString;

impl KLocalizedString {
    /// Set the given domain as application's main domain.
    ///
    /// [C++ API documentation](https://api-staging.kde.org/klocalizedstring.html#setApplicationDomain)
    pub fn set_application_domain(domain: &QByteArray) {
        ffi::set_application_domain(domain);
    }

    /// Get the application's main translation domain.
    ///
    /// [C++ API documentation](https://api-staging.kde.org/klocalizedstring.html#applicationDomain)
    pub fn application_domain() -> QByteArray {
        ffi::application_domain()
    }

    /// Create non-finalized translated string.
    ///
    /// [C++ API documentation](https://api-staging.kde.org/klocalizedstring.html#ki18n)
    pub fn ki18n(text: String) -> UniquePtr<Self> {
        ffi::ki18n(text)
    }

    /// Create non-finalized translated string with context.
    ///
    /// [C++ API documentation](https://api-staging.kde.org/klocalizedstring.html#ki18nc)
    pub fn ki18nc(context: String, text: String) -> UniquePtr<KLocalizedString> {
        ffi::ki18nc(context, text)
    }

    /// Create non-finalized translated string with plural.
    ///
    /// [C++ API documentation](https://api-staging.kde.org/klocalizedstring.html#ki18np)
    pub fn ki18np(singular: String, plural: String) -> UniquePtr<KLocalizedString> {
        ffi::ki18np(singular, plural)
    }

    /// Create non-finalized translated string with context and plural.
    ///
    /// [C++ API documentation](https://api-staging.kde.org/klocalizedstring.html#ki18ncp)
    pub fn ki18ncp(
        context: String,
        singular: String,
        plural: String,
    ) -> UniquePtr<KLocalizedString> {
        ffi::ki18ncp(context, singular, plural)
    }

    /// Create non-finalized translated string from domain.
    ///
    /// [C++ API documentation](https://api-staging.kde.org/klocalizedstring.html#ki18nd)
    pub fn ki18nd(domain: String, text: String) -> UniquePtr<KLocalizedString> {
        ffi::ki18nd(domain, text)
    }

    /// Create non-finalized translated string from domain with context.
    ///
    /// [C++ API documentation](https://api-staging.kde.org/klocalizedstring.html#ki18ndc)
    pub fn ki18ndc(domain: String, context: String, text: String) -> UniquePtr<KLocalizedString> {
        ffi::ki18ndc(domain, context, text)
    }

    /// Create non-finalized translated string from domain with plural.
    ///
    /// [C++ API documentation](https://api-staging.kde.org/klocalizedstring.html#ki18ndp)
    pub fn ki18ndp(
        domain: String,
        singular: String,
        plural: String,
    ) -> UniquePtr<KLocalizedString> {
        ffi::ki18ndp(domain, singular, plural)
    }

    /// Create non-finalized translated string from domain with context and plural.
    ///
    /// [C++ API documentation](https://api-staging.kde.org/klocalizedstring.html#ki18ndcp)
    pub fn ki18ndcp(
        domain: String,
        context: String,
        singular: String,
        plural: String,
    ) -> UniquePtr<KLocalizedString> {
        ffi::ki18ndcp(domain, context, singular, plural)
    }
}
