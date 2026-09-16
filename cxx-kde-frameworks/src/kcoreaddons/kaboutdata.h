// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

#ifndef _KABOUTDATA_RUST_BRIDGE_H_
#define _KABOUTDATA_RUST_BRIDGE_H_

#include <KAboutData>
#include "rust/cxx.h"

namespace rust::bridge::kaboutdata {

auto constructKAboutData(rust::Str componentName, rust::Str displayName, rust::Str version,
          rust::Str shortDescription, int license) -> std::unique_ptr<KAboutData>;

void setApplicationData(const KAboutData &aboutData);
} // namespace rust::bridge::kaboutdata

namespace rust::bridge::kaboutperson {
    auto constructKAboutPerson(rust::Str name, rust::Str task, rust::Str email_address, rust::Str web_address,
              const QUrl &avatar_url) -> KAboutPerson;
}


namespace rust {

template<>
struct IsRelocatable<KAboutPerson> : ::std::true_type
{};

}

#endif // _KABOUTDATA_RUST_BRIDGE_H_
