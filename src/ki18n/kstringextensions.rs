use cxx_qt_lib::QString;

use super::KLocalizedString;

// Convert this into macros later on with substitutions
pub fn i18n(text: &str) -> QString {
    return KLocalizedString::ki18n(text).to_qstring();
}

pub fn i18nc(context: &str, text: &str) -> QString {
    return KLocalizedString::ki18nc(context, text).to_qstring();
}

pub fn i18ncp(context: &str, singular: &str, plural: &str) -> QString {
    return KLocalizedString::ki18ncp(
        context,
        singular,
        plural,
    )
    .to_qstring();
}

pub fn i18nd(domain: &str, text: &str) -> QString {
    return KLocalizedString::ki18nd(domain, text).to_qstring();
}

pub fn i18ndc(domain: &str, context: &str, text: &str) -> QString {
    return KLocalizedString::ki18ndc(domain, context, text)
        .to_qstring();
}

pub fn i18ndcp(domain: &str, context: &str, singular: &str, plural: &str) -> QString {
    return KLocalizedString::ki18ndcp(
        domain,
        context,
        singular,
        plural,
    )
    .to_qstring();
}

pub fn i18ndp(domain: &str, singular: &str, plural: &str) -> QString {
    return KLocalizedString::ki18ndp(domain, singular, plural)
        .to_qstring();
}

pub fn i18np(singular: &str, plural: &str) -> QString {
    return KLocalizedString::ki18np(singular, plural).to_qstring();
}
