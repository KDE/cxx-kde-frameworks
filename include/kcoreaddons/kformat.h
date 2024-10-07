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

} // namespace kf6
} // namespace rust
