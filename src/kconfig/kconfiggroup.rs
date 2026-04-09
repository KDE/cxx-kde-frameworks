// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fell@gmx.de>
// SPDX-License-Identifier: MPL-2.0

use bitflags::bitflags;
use crate::kconfig::KConfig;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {

        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = cxx_qt_lib::QStringList;

        include!("cxx-kde-frameworks/kconfig.h");
        type KConfig = crate::kconfig::KConfig;

        include!("cxx-kde-frameworks/kconfiggroup.h");
        type KConfigGroup = super::KConfigGroup;
        type WriteConfigFlag;
        type WriteConfigFlags = super::WriteConfigFlags;

        fn readEntry(self: &KConfigGroup, key: &QString, aDefault: &QString) -> QString;

        fn writeEntry(self: &mut KConfigGroup, key: &QString, value: &QString, flags: WriteConfigFlags);

        fn exists(self: &KConfigGroup) -> bool;

        fn isValid(self: &KConfigGroup) -> bool;

        fn name(self: &KConfigGroup) -> QString;

        fn config(self: &KConfigGroup) -> *const KConfig;

    }

    #[namespace = "rust::kf6"]
    unsafe extern "C++" {
        fn config_mut(self_: &mut KConfigGroup) -> *mut KConfig;
    }

    #[derive(Debug)]
    #[repr(u32)]
    enum WriteConfigFlag {
        Persistent = 1,
        Global = 2,
        Localized = 4,
        Notify = 9,
        Normal = 1,
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "kconfiggroup_init_default"]
        fn construct() -> KConfigGroup;

        #[doc(hidden)]
        #[rust_name = "kconfiggroup_drop"]
        fn drop(format: &mut KConfigGroup);
    }

}

bitflags! {
    #[repr(C)]
    pub struct WriteConfigFlags: u32 {
        const Persistent = 1;
        const Global = 2;
        const Localized = 4;
        const Notify = 9;
        const Normal = 1;
    }
}

use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

pub use ffi::WriteConfigFlag;

pub struct KConfigGroup {
    _cspec: MaybeUninit<usize>,
    _cspec2: MaybeUninit<usize>,
}

impl Drop for KConfigGroup {
    fn drop(&mut self) {
        ffi::kconfiggroup_drop(self);
    }
}

impl KConfigGroup {
    pub fn config_mut(self: &mut KConfigGroup) -> *mut KConfig
    {
        ffi::config_mut(self)
    }
}

// Safety:

// Static checks on the C++ side ensure that KConfigGroup is trivial.
unsafe impl ExternType for KConfigGroup {
    type Id = type_id!("KConfigGroup");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for WriteConfigFlags {
    type Id = type_id!("WriteConfigFlags");
    type Kind = cxx::kind::Trivial;
}
