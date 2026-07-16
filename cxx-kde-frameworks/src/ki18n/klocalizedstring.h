// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0


#ifndef _KLOCALIZEDSTRING_RUST_BRIDGE_H_
#define _KLOCALIZEDSTRING_RUST_BRIDGE_H_

#include <KLocalizedString>

#include "rust/cxx.h"
#include "rustconv.h"

namespace rust::bridge::klocalizedstring {

rust::String applicationDomain();
void setApplicationDomain(rust::Str domain);

KLocalizedString r_ki18n(rust::String text);
KLocalizedString r_ki18nc(rust::String context, rust::String text);
KLocalizedString r_ki18ncp(rust::String context, rust::String singular, rust::String plural);
KLocalizedString r_ki18nd(rust::String domain, rust::String text);
KLocalizedString r_ki18ndc(rust::String domain, rust::String context, rust::String text);
KLocalizedString r_ki18ndcp(rust::String domain, rust::String context, rust::String singular, rust::String plural);
KLocalizedString r_ki18ndp(rust::String domain, rust::String singular, rust::String plural);
KLocalizedString r_ki18np(rust::String singular, rust::String plural);

} // namespace rust::bridge::klocalizedstring


namespace rust {

template <>
struct IsRelocatable<::KLocalizedString> : ::std::true_type
{
};

} // namespace rust


#endif // _KLOCALIZEDSTRING_RUST_BRIDGE_H_
