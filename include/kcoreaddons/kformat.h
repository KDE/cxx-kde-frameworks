#pragma once

#include <KFormat>

#include "rust/cxx.h"

namespace rust {

template<>
struct IsRelocatable<KFormat> : ::std::true_type
{};

namespace kf6 {

using KFormatBinaryUnitDialect = KFormat::BinaryUnitDialect;

using KFormatBinarySizeUnits = KFormat::BinarySizeUnits;

using KFormatUnitPrefix = KFormat::UnitPrefix;

QString formatSpelloutDuration(const KFormat &fmt, uint64_t msecs);
QString formatDecimalDuration(const KFormat &fmt, uint64_t msecs, int32_t decimalPlaces);

} // namespace kf6
} // namespace rust
