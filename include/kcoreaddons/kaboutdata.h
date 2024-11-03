// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: LGPL-2.0-or-later

#pragma once

#include <KAboutData>


#include "rust/cxx.h"

namespace rust {
namespace kf6 {

auto from(QString componentName, QString displayName, QString version,
          QString shortDescription, int license) -> std::unique_ptr<KAboutData>;

void setApplicationData(const KAboutData &aboutData);
} // namespace kf6
} // namespace rust
