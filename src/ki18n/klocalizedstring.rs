#[cxx_qt::bridge(cxx_file_stem = "klocalizedstring")]
mod ffi {

    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;

        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("kf6/klocalizedstring.h");

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
        fn r_ki18n(text: &str) -> UniquePtr<KLocalizedString>;

        #[rust_name = "ki18nc"]
        fn r_ki18nc(context: &str, text: &str) -> UniquePtr<KLocalizedString>;

        #[rust_name = "ki18ncp"]
        fn r_ki18ncp(context: &str, singular: &str, plural: &str) -> UniquePtr<KLocalizedString>;

        #[rust_name = "ki18nd"]
        fn r_ki18nd(domain: &str, text: &str) -> UniquePtr<KLocalizedString>;

        #[rust_name = "ki18ndc"]
        fn r_ki18ndc(domain: &str, context: &str, text: &str) -> UniquePtr<KLocalizedString>;

        #[rust_name = "ki18ndcp"]
        fn r_ki18ndcp(
            domain: &str,
            context: &str,
            singular: &str,
            plural: &str,
        ) -> UniquePtr<KLocalizedString>;

        #[rust_name = "ki18ndp"]
        fn r_ki18ndp(domain: &str, singular: &str, plural: &str) -> UniquePtr<KLocalizedString>;

        #[rust_name = "ki18np"]
        fn r_ki18np(singular: &str, plural: &str) -> UniquePtr<KLocalizedString>;
    }

    unsafe extern "C++" {
        #[rust_name = "to_qstring"]
        fn toString(self: &KLocalizedString) -> QString;
    }
}

use cxx::UniquePtr;
use cxx_qt_lib::{QByteArray, QString};

pub use ffi::KLocalizedString;

impl KLocalizedString {
    pub fn set_application_domain(domain: &QByteArray) {
        ffi::set_application_domain(domain);
    }

    pub fn application_domain() -> QByteArray {
        ffi::application_domain()
    }

    pub fn ki18n(text: &str) -> UniquePtr<Self> {
        ffi::ki18n(text)
    }

    pub fn ki18nc(context: &str, text: &str) -> UniquePtr<KLocalizedString> {
        ffi::ki18nc(context, text)
    }

    pub fn ki18ncp(context: &str, singular: &str, plural: &str) -> UniquePtr<KLocalizedString> {
        ffi::ki18ncp(context, singular, plural)
    }

    pub fn ki18nd(domain: &str, text: &str) -> UniquePtr<KLocalizedString> {
        ffi::ki18nd(domain, text)
    }

    pub fn ki18ndc(domain: &str, context: &str, text: &str) -> UniquePtr<KLocalizedString> {
        ffi::ki18ndc(domain, context, text)
    }

    pub fn ki18ndcp(
        domain: &str,
        context: &str,
        singular: &str,
        plural: &str,
    ) -> UniquePtr<KLocalizedString> {
        ffi::ki18ndcp(domain, context, singular, plural)
    }

    pub fn ki18ndp(domain: &str, singular: &str, plural: &str) -> UniquePtr<KLocalizedString> {
        ffi::ki18ndp(domain, singular, plural)
    }

    pub fn ki18np(singular: &str, plural: &str) -> UniquePtr<KLocalizedString> {
        ffi::ki18np(singular, plural)
    }
}

// Convert this into macros later on with substitutions
impl KLocalizedString {
    pub fn i18n(text: &str) -> QString {
        KLocalizedString::ki18n(text).to_qstring()
    }

    pub fn i18nc(context: &str, text: &str) -> QString {
        KLocalizedString::ki18nc(context, text).to_qstring()
    }

    pub fn i18ncp(context: &str, singular: &str, plural: &str) -> QString {
        KLocalizedString::ki18ncp(context, singular, plural).to_qstring()
    }

    pub fn i18nd(domain: &str, text: &str) -> QString {
        KLocalizedString::ki18nd(domain, text).to_qstring()
    }

    pub fn i18ndc(domain: &str, context: &str, text: &str) -> QString {
        KLocalizedString::ki18ndc(domain, context, text).to_qstring()
    }

    pub fn i18ndcp(domain: &str, context: &str, singular: &str, plural: &str) -> QString {
        KLocalizedString::ki18ndcp(domain, context, singular, plural).to_qstring()
    }

    pub fn i18ndp(domain: &str, singular: &str, plural: &str) -> QString {
        KLocalizedString::ki18ndp(domain, singular, plural).to_qstring()
    }

    pub fn i18np(singular: &str, plural: &str) -> QString {
        KLocalizedString::ki18np(singular, plural).to_qstring()
    }
}
