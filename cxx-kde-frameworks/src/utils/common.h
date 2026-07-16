// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#pragma once

namespace rust {
namespace bridge {

using c_void = void;

template<typename T, typename... Args>
T
construct(Args... args)
{
  return T(args...);
}

template<typename T>
void
drop(T& value)
{
  value.~T();
}

template<typename T, typename... Args>
std::unique_ptr<T>
make_unique(Args... args)
{
  return std::make_unique<T>(std::forward<Args>(args)...);
}

template<typename T, typename... Args>
std::shared_ptr<T>
make_shared(Args... args)
{
  return std::make_shared<T>(std::forward<Args>(args)...);
}

template<typename T, typename... Args>
T*
new_ptr(Args... args)
{
  return new T(std::forward<Args>(args)...);
}

} // namespace bridge
} // namespace rust