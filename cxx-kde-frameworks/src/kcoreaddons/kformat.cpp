// SPDX-FileCopyrightText: 2024 Nicolas Fella <nicolas.fell@gmx.de>
// SPDX-License-Identifier: MPL-2.0

#include "cxx-kde-frameworks/src/kcoreaddons/kformat.h"
#include "cxx-kde-frameworks/src/utils/assertion_utils.h"

#include "rustconv.h"

assert_alignment_and_size(KFormat, { ::std::size_t a0; });

static_assert(!::std::is_trivially_copy_assignable<KFormat>::value);
static_assert(!::std::is_trivially_copy_constructible<KFormat>::value);

static_assert(!::std::is_trivially_destructible<KFormat>::value);

//static_assert(QTypeInfo<KFormat>::isRelocatable);

namespace rust::bridge::kformat {

rust::String formatSpelloutDuration(const KFormat &fmt, uint64_t msecs) {
    return QStringToRustString(fmt.formatSpelloutDuration(msecs));
}

rust::String formatDecimalDuration(const KFormat &fmt, uint64_t msecs, int32_t decimalPlaces) {
    return QStringToRustString(fmt.formatDecimalDuration(msecs, decimalPlaces));
}

rust::String formatByteSize(const KFormat &fmt, double size, int32_t precision, KFormatBinaryUnitDialect dialect, KFormatBinarySizeUnits units){
    return QStringToRustString(fmt.formatByteSize(size, precision, dialect, units));
}

rust::String formatValue(const KFormat &fmt, double value, rust::Str unit, int32_t precision, KFormatUnitPrefix prefix, KFormatBinaryUnitDialect dialect){
    return QStringToRustString(fmt.formatValue(value, RustStrToQString(unit), precision, prefix, dialect));
}

} // namespace rust::bridge::kfromat
