// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use qtbridge_type_lib::QString;

#[cxx::bridge]
mod ffi {

    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/generated/core/qstring/cpp/qstring.h");
        type QString = super::QString;
        
        include!("cxx-kde-frameworks/src/ki18n/klocalizedstring.h");
        type KLocalizedString;
    }

    #[namespace = "rust::bridge::klocalizedstring"]
    unsafe extern "C++" {
        
        #[doc(hidden)]
        #[rust_name = "application_domain"]
        fn applicationDomain() -> String;
        
        #[doc(hidden)]
        #[rust_name = "set_application_domain"]
        fn setApplicationDomain(domain: &str);

        // #[doc(hidden)]
        // #[rust_name = "ki18n"]
        // fn r_ki18n(text: String) -> UniquePtr<KLocalizedString>;

        // #[doc(hidden)]
        // #[rust_name = "ki18nc"]
        // fn r_ki18nc(context: String, text: String) -> UniquePtr<KLocalizedString>;

        // #[doc(hidden)]
        // #[rust_name = "ki18np"]
        // fn r_ki18np(singular: String, plural: String) -> UniquePtr<KLocalizedString>;

        // #[doc(hidden)]
        // #[rust_name = "ki18ncp"]
        // fn r_ki18ncp(
        //     context: String,
        //     singular: String,
        //     plural: String,
        // ) -> UniquePtr<KLocalizedString>;

        // #[doc(hidden)]
        // #[rust_name = "ki18nd"]
        // fn r_ki18nd(domain: String, text: String) -> UniquePtr<KLocalizedString>;

        // #[doc(hidden)]
        // #[rust_name = "ki18ndc"]
        // fn r_ki18ndc(domain: String, context: String, text: String) -> UniquePtr<KLocalizedString>;

        // #[doc(hidden)]
        // #[rust_name = "ki18ndp"]
        // fn r_ki18ndp(
        //     domain: String,
        //     singular: String,
        //     plural: String,
        // ) -> UniquePtr<KLocalizedString>;

        // #[doc(hidden)]
        // #[rust_name = "ki18ndcp"]
        // fn r_ki18ndcp(
        //     domain: String,
        //     context: String,
        //     singular: String,
        //     plural: String,
        // ) -> UniquePtr<KLocalizedString>;

    }

    // unsafe extern "C++" {

    //     /// Finalizes and creates a translated [QString](cxx_qt_lib::QString).
    //     ///
    //     /// C++ counterpart: [KLocalizedString::toString()](https://api.kde.org/klocalizedstring.html#toString).
    //     #[rust_name = "to_qstring"]
    //     fn toString(self: &KLocalizedString) -> QString;
    // }
}


/// Class for producing and handling localized messages.
///
/// [C++ API documentation](https://api.kde.org/klocalizedstring.html)
pub use ffi::KLocalizedString;

impl KLocalizedString {
    
    /// Get the application's main translation domain.
    ///
    /// [C++ API documentation](https://api.kde.org/klocalizedstring.html#applicationDomain)
    pub fn application_domain() -> String {
        ffi::application_domain()
    }
    
    /// Set the given domain as application's main domain.
    ///
    /// [C++ API documentation](https://api.kde.org/klocalizedstring.html#setApplicationDomain)
    pub fn set_application_domain(domain: &str) {
        ffi::set_application_domain(domain);
    }

    // /// Create non-finalized translated string.
    // ///
    // /// [C++ API documentation](https://api.kde.org/klocalizedstring.html#ki18n)
    // pub fn ki18n(text: String) -> UniquePtr<Self> {
    //     ffi::ki18n(text)
    // }

    // /// Create non-finalized translated string with context.
    // ///
    // /// [C++ API documentation](https://api.kde.org/klocalizedstring.html#ki18nc)
    // pub fn ki18nc(context: String, text: String) -> UniquePtr<KLocalizedString> {
    //     ffi::ki18nc(context, text)
    // }

    // /// Create non-finalized translated string with plural.
    // ///
    // /// [C++ API documentation](https://api.kde.org/klocalizedstring.html#ki18np)
    // pub fn ki18np(singular: String, plural: String) -> UniquePtr<KLocalizedString> {
    //     ffi::ki18np(singular, plural)
    // }

    // /// Create non-finalized translated string with context and plural.
    // ///
    // /// [C++ API documentation](https://api.kde.org/klocalizedstring.html#ki18ncp)
    // pub fn ki18ncp(
    //     context: String,
    //     singular: String,
    //     plural: String,
    // ) -> UniquePtr<KLocalizedString> {
    //     ffi::ki18ncp(context, singular, plural)
    // }

    // /// Create non-finalized translated string from domain.
    // ///
    // /// [C++ API documentation](https://api.kde.org/klocalizedstring.html#ki18nd)
    // pub fn ki18nd(domain: String, text: String) -> UniquePtr<KLocalizedString> {
    //     ffi::ki18nd(domain, text)
    // }

    // /// Create non-finalized translated string from domain with context.
    // ///
    // /// [C++ API documentation](https://api.kde.org/klocalizedstring.html#ki18ndc)
    // pub fn ki18ndc(domain: String, context: String, text: String) -> UniquePtr<KLocalizedString> {
    //     ffi::ki18ndc(domain, context, text)
    // }

    // /// Create non-finalized translated string from domain with plural.
    // ///
    // /// [C++ API documentation](https://api.kde.org/klocalizedstring.html#ki18ndp)
    // pub fn ki18ndp(
    //     domain: String,
    //     singular: String,
    //     plural: String,
    // ) -> UniquePtr<KLocalizedString> {
    //     ffi::ki18ndp(domain, singular, plural)
    // }

    // /// Create non-finalized translated string from domain with context and plural.
    // ///
    // /// [C++ API documentation](https://api.kde.org/klocalizedstring.html#ki18ndcp)
    // pub fn ki18ndcp(
    //     domain: String,
    //     context: String,
    //     singular: String,
    //     plural: String,
    // ) -> UniquePtr<KLocalizedString> {
    //     ffi::ki18ndcp(domain, context, singular, plural)
    // }
}
