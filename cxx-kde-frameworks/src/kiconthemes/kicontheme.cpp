// SPDX-FileCopyrightText: 2024 Jonah Brüchert <jbb@kaidan.im>
// SPDX-License-Identifier: MPL-2.0

#include "cxx-kde-frameworks/kiconthemes/kicontheme.h"

namespace rust::bridge::kiconthemes {

void initTheme() {
    KIconTheme::initTheme();
}

} // namespace rust::brige::kiconthemes
