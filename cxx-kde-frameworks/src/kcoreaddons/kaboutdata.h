// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

#ifndef _KABOUTDATA_RUST_BRIDGE_H_
#define _KABOUTDATA_RUST_BRIDGE_H_

#include <KAboutData>
#include "rust/cxx.h"

namespace rust::bridge::kaboutdata {

auto from(QString componentName, QString displayName, QString version,
          QString shortDescription, int license) -> KAboutData;

void setApplicationData(const KAboutData &aboutData);

} // namespace rust::bridge::kaboutdata

namespace rust {

template<>
struct IsRelocatable<KAboutPerson> : ::std::true_type
{};

}

#endif // _KABOUTDATA_RUST_BRIDGE_H_
