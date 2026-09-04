// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

#include "cxx-kde-frameworks/ki18n/klocalizedstring.h"

namespace rust::bridge::klocalizedstring {

rust::String applicationDomain() {
    return QByteArrayToRustString(KLocalizedString::applicationDomain());
}

void setApplicationDomain(rust::Str domain) {
  KLocalizedString::setApplicationDomain(RustStrToQByteArray(domain));
}

KLocalizedString r_ki18n(rust::String text) {
  return KLocalizedString(ki18n(text.c_str()));
}

KLocalizedString r_ki18nc(rust::String context, rust::String text) {
  return KLocalizedString(ki18nc(context.c_str(), text.c_str()));
}

KLocalizedString r_ki18ncp(rust::String context, rust::String singular,
                           rust::String plural) {
  return KLocalizedString(
      ki18ncp(context.c_str(), singular.c_str(), plural.c_str()));
}

KLocalizedString r_ki18nd(rust::String domain, rust::String text) {
  return KLocalizedString(ki18nd(domain.c_str(), text.c_str()));
}

KLocalizedString r_ki18ndc(rust::String domain, rust::String context,
                           rust::String text) {
  return KLocalizedString(
      ki18ndc(domain.c_str(), context.c_str(), text.c_str()));
}

KLocalizedString r_ki18ndcp(rust::String domain, rust::String context,
                            rust::String singular, rust::String plural) {
  return KLocalizedString(ki18ndcp(domain.c_str(), context.c_str(),
                                   singular.c_str(), plural.c_str()));
}

KLocalizedString r_ki18ndp(rust::String domain, rust::String singular,
                           rust::String plural) {
  return KLocalizedString(
      ki18ndp(domain.c_str(), singular.c_str(), plural.c_str()));
}

KLocalizedString r_ki18np(rust::String singular, rust::String plural) {
  return KLocalizedString(ki18np(singular.c_str(), plural.c_str()));
}

} // namespace rust::bridge:klocalizedstring
