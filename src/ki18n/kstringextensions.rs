// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use cxx_qt_lib::QString;

use super::KLocalizedString;

// Convert this into macros later on with substitutions
pub fn i18n(text: &str) -> QString {
    return KLocalizedString::ki18n(text.to_string()).to_qstring();
}

pub fn i18nc(context: &str, text: &str) -> QString {
    return KLocalizedString::ki18nc(context.to_string(), text.to_string()).to_qstring();
}

pub fn i18ncp(context: &str, singular: &str, plural: &str) -> QString {
    return KLocalizedString::ki18ncp(
        context.to_string(),
        singular.to_string(),
        plural.to_string(),
    )
    .to_qstring();
}

pub fn i18nd(domain: &str, text: &str) -> QString {
    return KLocalizedString::ki18nd(domain.to_string(), text.to_string()).to_qstring();
}

pub fn i18ndc(domain: &str, context: &str, text: &str) -> QString {
    return KLocalizedString::ki18ndc(domain.to_string(), context.to_string(), text.to_string())
        .to_qstring();
}

pub fn i18ndcp(domain: &str, context: &str, singular: &str, plural: &str) -> QString {
    return KLocalizedString::ki18ndcp(
        domain.to_string(),
        context.to_string(),
        singular.to_string(),
        plural.to_string(),
    )
    .to_qstring();
}

pub fn i18ndp(domain: &str, singular: &str, plural: &str) -> QString {
    return KLocalizedString::ki18ndp(domain.to_string(), singular.to_string(), plural.to_string())
        .to_qstring();
}

pub fn i18np(singular: &str, plural: &str) -> QString {
    return KLocalizedString::ki18np(singular.to_string(), plural.to_string()).to_qstring();
}
