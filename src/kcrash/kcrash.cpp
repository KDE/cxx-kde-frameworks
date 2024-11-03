// SPDX-FileCopyrightText: 2024 Jonah Brüchert <jbb@kaidan.im>
// SPDX-License-Identifier: LGPL-2.0-or-later

#include "cxx-kde-frameworks/kcrash.h"

namespace rust {
namespace kf6 {

void initializeKCrash() {
    KCrash::initialize();
}

} // namespace kf6
} // namespace rust
