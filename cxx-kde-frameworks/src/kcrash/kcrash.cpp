// SPDX-FileCopyrightText: 2024 Jonah Brüchert <jbb@kaidan.im>
// SPDX-License-Identifier: MPL-2.0

#include "cxx-kde-frameworks/kcrash/kcrash.h"

namespace rust {
namespace kf6 {

void initializeKCrash() {
    KCrash::initialize();
}

} // namespace kf6
} // namespace rust
