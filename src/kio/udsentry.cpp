// SPDX-FileCopyrightText: 2024 Nicolas Fella <nicolas.fell@gmx.de>
// SPDX-License-Identifier: MPL-2.0

#include "cxx-kde-frameworks/udsentry.h"
#include <cxx-qt-lib/assertion_utils.h>

assert_alignment_and_size(UDSEntry, { ::std::size_t a0; });

static_assert(!::std::is_trivially_copy_assignable<UDSEntry>::value);
static_assert(!::std::is_trivially_copy_constructible<UDSEntry>::value);

static_assert(!::std::is_trivially_destructible<UDSEntry>::value);

static_assert(QTypeInfo<UDSEntry>::isRelocatable);
