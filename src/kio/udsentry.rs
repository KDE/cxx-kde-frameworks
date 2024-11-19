// SPDX-FileCopyrightText: 2024 Nicolas Fella <nicolas.fell@gmx.de>
// SPDX-License-Identifier: MPL-2.0

#[cxx_qt::bridge]
mod ffi {

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qlist.h");
        type QList_u32 = cxx_qt_lib::QList<u32>;
    }

    unsafe extern "C++" {
        include!("cxx-kde-frameworks/udsentry.h");

        type UDSEntry = super::UDSEntry;

        #[rust_name = "string_value"]
        fn stringValue(self: &UDSEntry, field: u32) -> QString;

        // long long numberValue(uint field, long long defaultValue = 0) const;
        // #[rust_name = "number_value"]
        // fn numberValue(self: &UDSEntry, field: u32, defaultValue: i64) -> i64;

        // bool isDir() const;
        #[rust_name = "is_dir"]
        fn isDir(self: &UDSEntry) -> bool;

        // bool isLink() const;
        #[rust_name = "is_link"]
        fn isLink(self: &UDSEntry) -> bool;

        // void reserve(int size);
        fn reserve(self: &mut UDSEntry, size: i32);

        // void fastInsert(uint field, const QString &value);
        #[rust_name = "fast_insert"]
        fn fastInsert(self: &mut UDSEntry, field: u32, value: &QString);

        // void fastInsert(uint field, long long l);

        // int count() const;
        fn count(self: &UDSEntry) -> i32;

        // bool contains(uint field) const;
        fn contains(self: &UDSEntry, field: u32) -> bool;

        // QList<uint> fields() const;
        fn fields(self: &UDSEntry) -> QList_u32;

        // void clear();
        fn clear(self: &mut UDSEntry);

        // void replace(uint field, const QString &value);
        fn replace(self: &mut UDSEntry, field: u32, value: &QString);

        // void replace(uint field, long long l);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "udsentry_init_default"]
        fn construct() -> UDSEntry;

        #[doc(hidden)]
        #[rust_name = "udsentry_drop"]
        fn drop(format: &mut UDSEntry);
    }
}

use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[repr(C)]
pub struct UDSEntry {
    _cspec: MaybeUninit<usize>,
}

impl Default for UDSEntry {
    fn default() -> UDSEntry {
        ffi::udsentry_init_default()
    }
}

impl Drop for UDSEntry {
    fn drop(&mut self) {
        ffi::udsentry_drop(self);
    }
}

// Safety:

// Static checks on the C++ side ensure that UDSEntry is trivial.
unsafe impl ExternType for UDSEntry {
    type Id = type_id!("UDSEntry");
    type Kind = cxx::kind::Trivial;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let uds = UDSEntry::default();

        assert_eq!(uds.is_dir(), false);

        assert_eq!(uds.is_link(), false);

        assert_eq!(uds.count(), 0);
    }
}
