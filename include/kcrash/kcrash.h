// SPDX-FileCopyrightText: 2024 Jonah Brüchert <jbb@kaidan.im>
// SPDX-License-Identifier: LGPL-2.0-or-later

#pragma once

#include <KCrash>

#include "rust/cxx.h"

namespace rust {
namespace kf6 {

void initializeKCrash();

} // namespace kf6
} // namespace rust
