// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fell@gmx.de>
// SPDX-License-Identifier: MPL-2.0

#include "cxx-kde-frameworks/kconfiggroup.h"
#include <cxx-qt-lib/assertion_utils.h>

assert_alignment_and_size(KConfigGroup, { ::std::size_t a0; ::std::size_t a1; });

static_assert(!::std::is_trivially_copy_assignable<KConfigGroup>::value);
static_assert(!::std::is_trivially_copy_constructible<KConfigGroup>::value);

static_assert(!::std::is_trivially_destructible<KConfigGroup>::value);

// static_assert(QTypeInfo<KConfigGroup::isRelocatable);
