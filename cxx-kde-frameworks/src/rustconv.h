// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef RUST_CONV_H
#define RUST_CONV_H
#include <QByteArray>
#include <QMetaType>
#include <QStringList>
#include <type_traits>
#include <vector>
#include "rust/cxx.h"


inline QByteArray RustStrToQByteArray(const rust::Str src)
{
    // Return byte array of UTF8 chars
    return { src.data(), static_cast<qsizetype>(src.size()) };
}

inline QByteArray RustByteSliceToQByteArray(const rust::Slice<const uint8_t> src)
{
    return QByteArray(reinterpret_cast<const char*>(src.data()), static_cast<qsizetype>(src.size()));
}

inline QString RustStrToQString(const rust::Str src)
{
    return QString::fromUtf8(RustStrToQByteArray(src));
}

inline QByteArray RustStringToQByteArray(const rust::String& src)
{
    return { src.data(), static_cast<qsizetype>(src.size()) };
}

inline QString RustStringToQString(const rust::String& src)
{
    return QString::fromUtf8(RustStringToQByteArray(src));
}

inline rust::String QByteArrayToRustString(const QByteArray& src)
{
    // assuming that byte array holds string in UTF8 encoding
    return { src.data(), static_cast<size_t>(src.size()) };
}

inline rust::String QStringToRustString(const QString& src)
{
    return QByteArrayToRustString(src.toUtf8());
}

inline rust::String CStrToRustString(const char* src)
{
    return QStringToRustString(QString::fromLocal8Bit(src));
}

inline QStringList RustStringListToQStringList(const rust::Vec<rust::String>& sl)
{
    QStringList result;
    result.reserve(sl.size());
    for (const auto& str : sl)
        result.push_back(RustStrToQString(str));
    return result;
}

inline rust::Vec<rust::String> QStringListToRustStringList(const QStringList& sl)
{
    rust::Vec<rust::String> result;
    result.reserve(static_cast<size_t>(sl.size()));
    for (const auto& qstr: sl)
    {
        result.emplace_back(QStringToRustString(qstr));
    }
    return result;
}

template <typename T>
inline QByteArray RustSliceToQByteArray(rust::Slice<const T> slice)
{
    return { reinterpret_cast<const char*>(slice.data()), static_cast<qsizetype>(slice.size()) };
}

template <typename T>
inline QSpan<const T> RustSliceToQSpan(rust::Slice<const T> slice)
{
    return QSpan(slice.begin(), slice.end());
}

template <typename RustContainerT, typename CppT = std::remove_const_t<typename RustContainerT::value_type> >
inline std::vector<CppT> RustContainerToCppVector(RustContainerT src)
{
    std::vector<CppT> result = { src.begin(), src.end() };
    return result;
}

#endif // RUST_CONV_H
