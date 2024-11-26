// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

#pragma once

#include <KLocalizedContext>

#include <QtQml/QQmlApplicationEngine>
#include <QtQml/QQmlContext>
#include <QtQml/QQmlEngine>

namespace rust {

namespace kf6 {
void initializeEngine(QQmlEngine &engine);

}

} // namespace rust
