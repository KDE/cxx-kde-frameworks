// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fell@gmx.de>
// SPDX-License-Identifier: MPL-2.0

#pragma once

#include <KConfig>

namespace rust::kf6::kconfig {
    std::unique_ptr<KConfig> from(QString file);

    using KConfigAccessMode = KConfigBase::AccessMode;

}
