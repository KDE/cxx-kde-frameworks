// SPDX-FileCopyrightText: 2026 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

#pragma once

#include <KLocalizedQmlContext>
#include <QtQml/QQmlEngine>

namespace rust {
namespace kf6 {
void setupLocalizedContext(QQmlEngine &engine);

}
} // namespace rust
