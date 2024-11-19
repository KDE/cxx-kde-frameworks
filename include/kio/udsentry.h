// SPDX-FileCopyrightText: 2024 Nicolas Fella <nicolas.fell@gmx.de>
// SPDX-License-Identifier: MPL-2.0

#pragma once

#include <KIO/UDSEntry>

using UDSEntry = KIO::UDSEntry;

#include "rust/cxx.h"

namespace rust {

template<>
struct IsRelocatable<UDSEntry> : ::std::true_type
{};

}
