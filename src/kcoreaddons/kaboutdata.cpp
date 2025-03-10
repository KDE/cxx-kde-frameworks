// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

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

} // namespace kf6
} // namespace rust


assert_alignment_and_size(KAboutPerson, { ::std::size_t a0; });

static_assert(!::std::is_trivially_copy_assignable<KAboutPerson>::value);
static_assert(!::std::is_trivially_copy_constructible<KAboutPerson>::value);

static_assert(!::std::is_trivially_destructible<KAboutPerson>::value);

static_assert(QTypeInfo<KAboutPerson>::isRelocatable);
