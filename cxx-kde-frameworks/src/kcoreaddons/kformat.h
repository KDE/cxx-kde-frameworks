// SPDX-FileCopyrightText: 2024 Nicolas Fella <nicolas.fell@gmx.de>
// SPDX-License-Identifier: MPL-2.0


#ifndef _KFORMAT_RUST_BRIDGE_H_
#define  _KFORMAT_RUST_BRIDGE_H_

#include <KFormat>

#include "rust/cxx.h"

namespace rust::bridge::kformat {

using KFormatBinaryUnitDialect = KFormat::BinaryUnitDialect;

using KFormatBinarySizeUnits = KFormat::BinarySizeUnits;

using KFormatUnitPrefix = KFormat::UnitPrefix;

rust::String formatSpelloutDuration(const KFormat &fmt, uint64_t msecs);
rust::String formatDecimalDuration(const KFormat &fmt, uint64_t msecs, int32_t decimalPlaces);
rust::String formatByteSize(const KFormat &fmt, double size, int32_t precision, KFormatBinaryUnitDialect dialect, KFormatBinarySizeUnits units);
rust::String formatValue(const KFormat &fmt, double value, rust::Str unit, int32_t precision, KFormatUnitPrefix prefix, KFormatBinaryUnitDialect dialect);

} // namespace rust::bridge::kformat


namespace rust {

template<>
struct IsRelocatable<KFormat> : ::std::true_type
{};

} // namespace rust

#endif // _KFORMAT_RUST_BRIDGE_H_
