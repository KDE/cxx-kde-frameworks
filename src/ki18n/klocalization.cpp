// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

#include "cxx-kde-frameworks/klocalization.h"

namespace rust {
namespace kf6 {

void setupLocalizedContext(QQmlEngine &engine) {
  KLocalization::setupLocalizedContext(&engine);
}

} // namespace kf6
} // namespace rust
