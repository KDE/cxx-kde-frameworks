// SPDX-FileCopyrightText: 2024 Jonah Brüchert <jbb@kaidan.im>
// SPDX-License-Identifier: MPL-2.0

#ifndef _KCRASH_RUST_BRIDGE_H_
#define _KCRASH_RUST_BRIDGE_H_

#include <KCrash>

#include "rust/cxx.h"

namespace rust::bridge::kcrash {

void initializeKCrash();

} // namespace rust::bridge::klocalization

#endif // _KCRASH_RUST_BRIDGE_H_
