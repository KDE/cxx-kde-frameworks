#include "cxx-kde-frameworks/klocalizedstring.h"

namespace rust {
namespace kf6 {

void setApplicationDomain(const QByteArray &domain) {
  KLocalizedString::setApplicationDomain(domain);
}

auto applicationDomain() -> QByteArray {
  return KLocalizedString::applicationDomain();
}

auto r_ki18n(rust::Str text) -> std::unique_ptr<KLocalizedString> {
  return std::make_unique<KLocalizedString>(ki18n(text.data()));
}

auto r_ki18nc(rust::Str context,
              rust::Str text) -> std::unique_ptr<KLocalizedString> {
  return std::make_unique<KLocalizedString>(
      ki18nc(context.data(), text.data()));
}
auto r_ki18ncp(rust::Str context, rust::Str singular,
               rust::Str plural) -> std::unique_ptr<KLocalizedString> {
  return std::make_unique<KLocalizedString>(
      ki18ncp(context.data(), singular.data(), plural.data()));
}
auto r_ki18nd(rust::Str domain,
              rust::Str text) -> std::unique_ptr<KLocalizedString> {
  return std::make_unique<KLocalizedString>(
      ki18nd(domain.data(), text.data()));
}
auto r_ki18ndc(rust::Str domain, rust::Str context,
               rust::Str text) -> std::unique_ptr<KLocalizedString> {
  return std::make_unique<KLocalizedString>(
      ki18ndc(domain.data(), context.data(), text.data()));
}
auto r_ki18ndcp(rust::Str domain, rust::Str context,
                rust::Str singular,
                rust::Str plural) -> std::unique_ptr<KLocalizedString> {
  return std::make_unique<KLocalizedString>(ki18ndcp(
      domain.data(), context.data(), singular.data(), plural.data()));
}
auto r_ki18ndp(rust::Str domain, rust::Str singular,
               rust::Str plural) -> std::unique_ptr<KLocalizedString> {
  return std::make_unique<KLocalizedString>(
      ki18ndp(domain.data(), singular.data(), plural.data()));
}
auto r_ki18np(rust::Str singular,
              rust::Str plural) -> std::unique_ptr<KLocalizedString> {
  return std::make_unique<KLocalizedString>(
      ki18np(singular.data(), plural.data()));
}

} // namespace kf6
} // namespace rust
