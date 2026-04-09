// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fell@gmx.de>
// SPDX-License-Identifier: MPL-2.0

use crate::kconfig::KConfigGroup;
use cxx::{type_id, ExternType, UniquePtr};
use cxx_qt_lib::QString;
use std::mem::MaybeUninit;

#[cxx_qt::bridge()]
mod ffi {

    #[namespace = "rust::kf6::kconfig"]
    unsafe extern "C++" {
        fn from(file: QString) -> UniquePtr<KConfig>;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = cxx_qt_lib::QStringList;

        include!("cxx-kde-frameworks/kconfig.h");
        type KConfig;

        include!("cxx-kde-frameworks/kconfiggroup.h");
        type KConfigGroup = crate::kconfig::KConfigGroup;

        fn groupList(self: &KConfig) -> QStringList;

        fn group(self: Pin<&mut KConfig>, group: &QString) -> KConfigGroup;

        fn hasGroup(self: &KConfig, group: &QString) -> bool;

        fn sync(self: Pin<&mut KConfig>) -> bool;

        fn markAsClean(self: Pin<&mut KConfig>);

        fn isImmutable(self: &KConfig) -> bool;

        fn isGroupImmutable(self: &KConfig, group: &QString) -> bool;

        fn name(self: &KConfig) -> QString;

        fn isConfigWritable(self: Pin<&mut KConfig>, warnUser: bool) -> bool;

        fn copyFrom(self: &KConfig, config: &KConfig);

        fn checkUpdate(self: Pin<&mut KConfig>, id: &QString, updateFile: &QString);

        fn reparseConfiguration(self: Pin<&mut KConfig>);

        fn addConfigSources(self: Pin<&mut KConfig>, sources: &QStringList);
    }

    #[namespace = "rust::kf6"]
    unsafe extern "C++" {}

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");
    }
}

pub use ffi::KConfig;

impl KConfig {
    pub fn from(file: QString) -> UniquePtr<KConfig> {
        ffi::from(file)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use cxx_qt_lib::QString;

    #[test]
    fn test_add() {
        let mut config = KConfig::from(QString::from("kdeglobals"));

        let group = config.as_mut().unwrap().group(&QString::from("General"));

        let s = group.readEntry(&QString::from("ColorSchemeHash"), &QString::from("12345"));
    }
}
