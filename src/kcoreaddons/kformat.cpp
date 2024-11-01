// SPDX-FileCopyrightText: 2024 Nicolas Fella <nicolas.fell@gmx.de>
// SPDX-License-Identifier: MPL-2.0

#include "cxx-kde-frameworks/kformat.h"
#include <cxx-qt-lib/assertion_utils.h>

assert_alignment_and_size(KFormat, { ::std::size_t a0; });

static_assert(!::std::is_trivially_copy_assignable<KFormat>::value);
static_assert(!::std::is_trivially_copy_constructible<KFormat>::value);

static_assert(!::std::is_trivially_destructible<KFormat>::value);

//static_assert(QTypeInfo<KFormat>::isRelocatable);

namespace rust {
namespace kf6 {

QString formatSpelloutDuration(const KFormat &fmt, uint64_t msecs) {
    return fmt.formatSpelloutDuration(msecs);
}

QString formatDecimalDuration(const KFormat &fmt, uint64_t msecs, int32_t decimalPlaces) {
    return fmt.formatDecimalDuration(msecs, decimalPlaces);
}

} // namespace kf6
} // namespace rust
