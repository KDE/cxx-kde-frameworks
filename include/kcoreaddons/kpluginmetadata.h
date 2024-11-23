// SPDX-FileCopyrightText: 2024 Nicolas Fella <nicolas.fell@gmx.de>
// SPDX-License-Identifier: MPL-2.0

#pragma once

#include <KPluginMetaData>

#include "rust/cxx.h"

namespace rust {

template<>
struct IsRelocatable<KPluginMetaData> : ::std::true_type
{};

}
