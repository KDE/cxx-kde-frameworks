#pragma once

#include <KLocalizedString>

#include "rust/cxx.h"

namespace rust {
namespace kf6 {
void setApplicationDomain(const QByteArray &domain);

auto applicationDomain() -> QByteArray;

auto r_ki18n(rust::Str text) -> std::unique_ptr<KLocalizedString>;
auto r_ki18nc(rust::Str context, rust::Str text)
    -> std::unique_ptr<KLocalizedString>;
auto r_ki18ncp(rust::Str context, rust::Str singular, rust::Str plural)
    -> std::unique_ptr<KLocalizedString>;
auto r_ki18nd(rust::Str domain, rust::Str text)
    -> std::unique_ptr<KLocalizedString>;
auto r_ki18ndc(rust::Str domain, rust::Str context, rust::Str text)
    -> std::unique_ptr<KLocalizedString>;
auto r_ki18ndcp(rust::Str domain, rust::Str context,
                rust::Str singular, rust::Str plural)
    -> std::unique_ptr<KLocalizedString>;
auto r_ki18ndp(rust::Str domain, rust::Str singular, rust::Str plural)
    -> std::unique_ptr<KLocalizedString>;
auto r_ki18np(rust::Str singular, rust::Str plural)
    -> std::unique_ptr<KLocalizedString>;

} // namespace kf6
} // namespace rust
