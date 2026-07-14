// SPDX-FileCopyrightText: 2026 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

#ifndef _KLOCALIZATION_RUST_BRIDGE_H_
#define _KLOCALIZATION_RUST_BRIDGE_H_

#include <KLocalizedQmlContext>
#include <QtQml/QQmlApplicationEngine>
#include "rust/cxx.h"


namespace rust::bridge::klocalization {

void inlineCppFn_setupLocalizedContext(QQmlApplicationEngine &engine);

} // namespace rust::bridge::klocalization


#endif // _KLOCALIZATION_RUST_BRIDGE_H_
