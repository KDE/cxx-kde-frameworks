// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fell@gmx.de>
// SPDX-License-Identifier: MPL-2.0

#pragma once

#include <KConfigGroup>
#include "rust/cxx.h"

namespace rust {

    template<>
    struct IsRelocatable<KConfigGroup> : ::std::true_type
    {};

    namespace kf6 {
    }

}

using WriteConfigFlag = KConfigBase::WriteConfigFlag;
using WriteConfigFlags = KConfigBase::WriteConfigFlags;
