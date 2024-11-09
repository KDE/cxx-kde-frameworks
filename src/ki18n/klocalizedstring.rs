// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

#[cxx_qt::bridge]
mod ffi {

    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;

        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-kde-frameworks/klocalizedstring.h");

        #[qobject]
        type KLocalizedString;

    }

    #[namespace = "rust::kf6"]
    unsafe extern "C++" {
        #[rust_name = "set_application_domain"]
        fn setApplicationDomain(domain: &QByteArray);

        #[rust_name = "application_domain"]
        fn applicationDomain() -> QByteArray;

        #[rust_name = "ki18n"]
        fn r_ki18n(text: String) -> UniquePtr<KLocalizedString>;

        #[rust_name = "ki18nc"]
        fn r_ki18nc(context: String, text: String) -> UniquePtr<KLocalizedString>;

        #[rust_name = "ki18ncp"]
        fn r_ki18ncp(
            context: String,
            singular: String,
            plural: String,
        ) -> UniquePtr<KLocalizedString>;

        #[rust_name = "ki18nd"]
        fn r_ki18nd(domain: String, text: String) -> UniquePtr<KLocalizedString>;

        #[rust_name = "ki18ndc"]
        fn r_ki18ndc(domain: String, context: String, text: String) -> UniquePtr<KLocalizedString>;

        #[rust_name = "ki18ndcp"]
        fn r_ki18ndcp(
            domain: String,
            context: String,
            singular: String,
            plural: String,
        ) -> UniquePtr<KLocalizedString>;

        #[rust_name = "ki18ndp"]
        fn r_ki18ndp(
            domain: String,
            singular: String,
            plural: String,
        ) -> UniquePtr<KLocalizedString>;

        #[rust_name = "ki18np"]
        fn r_ki18np(singular: String, plural: String) -> UniquePtr<KLocalizedString>;
    }

    unsafe extern "C++" {
        #[rust_name = "to_qstring"]
        fn toString(self: &KLocalizedString) -> QString;
    }
}

use cxx::UniquePtr;
use cxx_qt_lib::QByteArray;

pub use ffi::KLocalizedString;

impl KLocalizedString {
    pub fn set_application_domain(domain: &QByteArray) {
        ffi::set_application_domain(domain);
    }

    pub fn application_domain() -> QByteArray {
        ffi::application_domain()
    }

    pub fn ki18n(text: String) -> UniquePtr<Self> {
        ffi::ki18n(text)
    }

    pub fn ki18nc(context: String, text: String) -> UniquePtr<KLocalizedString> {
        ffi::ki18nc(context, text)
    }

    pub fn ki18ncp(
        context: String,
        singular: String,
        plural: String,
    ) -> UniquePtr<KLocalizedString> {
        ffi::ki18ncp(context, singular, plural)
    }

    pub fn ki18nd(domain: String, text: String) -> UniquePtr<KLocalizedString> {
        ffi::ki18nd(domain, text)
    }

    pub fn ki18ndc(domain: String, context: String, text: String) -> UniquePtr<KLocalizedString> {
        ffi::ki18ndc(domain, context, text)
    }

    pub fn ki18ndcp(
        domain: String,
        context: String,
        singular: String,
        plural: String,
    ) -> UniquePtr<KLocalizedString> {
        ffi::ki18ndcp(domain, context, singular, plural)
    }

    pub fn ki18ndp(
        domain: String,
        singular: String,
        plural: String,
    ) -> UniquePtr<KLocalizedString> {
        ffi::ki18ndp(domain, singular, plural)
    }

    pub fn ki18np(singular: String, plural: String) -> UniquePtr<KLocalizedString> {
        ffi::ki18np(singular, plural)
    }
}
