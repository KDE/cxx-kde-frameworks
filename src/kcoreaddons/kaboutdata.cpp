// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

#include "cxx-kde-frameworks/kaboutdata.h"

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
