// SPDX-FileCopyrightText: 2024 Jonah Brüchert <jbb@kaidan.im>
// SPDX-License-Identifier: MIT or Apache-2.0

#include "cxx-kde-frameworks/kcrash.h"

namespace rust {
namespace kf6 {

void initializeKCrash() {
    KCrash::initialize();
}

} // namespace kf6
} // namespace rust
