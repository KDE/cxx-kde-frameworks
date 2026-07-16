// SPDX-FileCopyrightText: 2024 Nicolas Fella <nicolas.fell@gmx.de>
// SPDX-License-Identifier: MPL-2.0

#ifndef _KPLUGINMETADATA_RUST_BRIDGE_H_
#define _KPLUGINMETADATA_RUST_BRIDGE_H_

#include <KPluginMetaData>

#include "rust/cxx.h"

namespace rust {

template<>
struct IsRelocatable<KPluginMetaData> : ::std::true_type
{};

}

#endif // _KPLUGINMETADATA_RUST_BRIDGE_H_