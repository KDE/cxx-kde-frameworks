#include "cxx-kde-frameworks/kaboutdata.h"
#include <cxx-qt-lib/assertion_utils.h>

namespace rust {
namespace kf6 {

auto from(QString componentName, QString displayName, QString version,
          QString shortDescription,
          int license) -> std::unique_ptr<KAboutData> {
  return std::make_unique<KAboutData>(
      componentName, displayName, version, shortDescription,
      static_cast<KAboutLicense::LicenseKey>(license));
}

void setApplicationData(const KAboutData &aboutData) {
  KAboutData::setApplicationData(aboutData);
}

KAboutPerson kaboutperson_init_default()
{
    return KAboutPerson();
}

KAboutPerson kaboutperson_init(const QString &name, const QString &task, const QString &emailAddress, const QString &websiteAddress, const QUrl &avatarUrl)
{
    return KAboutPerson(name, task, emailAddress, websiteAddress, avatarUrl);
}

} // namespace kf6
} // namespace rust


assert_alignment_and_size(KAboutPerson, { ::std::size_t a0; });

static_assert(!::std::is_trivially_copy_assignable<KAboutPerson>::value);
static_assert(!::std::is_trivially_copy_constructible<KAboutPerson>::value);

static_assert(!::std::is_trivially_destructible<KAboutPerson>::value);

static_assert(QTypeInfo<KAboutPerson>::isRelocatable);
