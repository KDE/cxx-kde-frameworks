// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

#include "cxx-kde-frameworks/kcoreaddons/kaboutdata.h"
#include <cxx-qt-lib/assertion_utils.h>
#include "rustconv.h"

namespace rust::bridge::kaboutdata {

auto constructKAboutData(rust::Str componentName, rust::Str displayName, rust::Str version,
          rust::Str shortDescription, int license) -> std::unique_ptr<KAboutData> {
  return std::make_unique<KAboutData>(
      RustStrToQString(componentName),
      RustStrToQString(displayName),
      RustStrToQString(version),
      RustStrToQString(shortDescription),
      static_cast<KAboutLicense::LicenseKey>(license));
}

void setApplicationData(const KAboutData &aboutData) {
  KAboutData::setApplicationData(aboutData);
}

} // namespace rust::bridge::kaboutdata


namespace rust::bridge::kaboutperson {
    auto constructKAboutPerson(rust::Str name, rust::Str task, rust::Str email_address, rust::Str web_address, const QUrl &avatar_url) -> KAboutPerson {
        return KAboutPerson(
            RustStrToQString(name),
            RustStrToQString(task),
            RustStrToQString(email_address),
            RustStrToQString(web_address),
            avatar_url
        );
    }

}


assert_alignment_and_size(KAboutPerson, { ::std::size_t a0; });

static_assert(!::std::is_trivially_copy_assignable<KAboutPerson>::value);
static_assert(!::std::is_trivially_copy_constructible<KAboutPerson>::value);

static_assert(!::std::is_trivially_destructible<KAboutPerson>::value);

static_assert(QTypeInfo<KAboutPerson>::isRelocatable);
