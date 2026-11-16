// SPDX-FileCopyrightText: 2024 Nicolas Fella <nicolas.fell@gmx.de>
// SPDX-License-Identifier: MPL-2.0

#include "cxx-kde-frameworks/kpluginmetadata.h"
#include <cxx-qt-lib/assertion_utils.h>

assert_alignment_and_size(KPluginMetaData, { ::std::size_t a0; });

static_assert(!::std::is_trivially_copy_assignable<KPluginMetaData>::value);
static_assert(!::std::is_trivially_copy_constructible<KPluginMetaData>::value);

static_assert(!::std::is_trivially_destructible<KPluginMetaData>::value);

static_assert(QTypeInfo<KPluginMetaData>::isRelocatable);
