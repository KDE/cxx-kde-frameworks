// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fell@gmx.de>
// SPDX-License-Identifier: MPL-2.0

#include "kconfig.h"

namespace rust::kf6::kconfig {
    std::unique_ptr<KConfig> from(QString file, KConfig::OpenFlags mode)
    {
        return std::make_unique<KConfig>(file, mode);
    }
}
