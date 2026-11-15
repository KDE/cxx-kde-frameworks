// SPDX-FileCopyrightText: 2024 Jonah Brüchert <jbb@kaidan.im>
// SPDX-License-Identifier: MPL-2.0

#include "cxx-kde-frameworks/kiconthemes/kicontheme.h"

namespace rust {
namespace kf6 {

void initIcons() {
    KIconTheme::initTheme();
}

} // namespace kf6
} // namespace rust
