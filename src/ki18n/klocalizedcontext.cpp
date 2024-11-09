// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

#include "cxx-kde-frameworks/klocalizedcontext.h"

namespace rust {
namespace kf6 {

void initializeEngine(QQmlEngine &engine) {
  engine.rootContext()->setContextObject(new KLocalizedContext(&engine));
}

} // namespace kf6
} // namespace rust
