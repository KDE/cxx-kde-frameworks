// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fell@gmx.de>
// SPDX-License-Identifier: MPL-2.0

use cxx::{UniquePtr};
use cxx_qt_lib::QString;
use crate::kconfig::{WriteConfigFlags};
use cxx::{type_id, ExternType};
use bitflags::bitflags;

#[cxx_qt::bridge()]
mod ffi {

    #[namespace = "rust::kf6::kconfig"]
    #[derive(Debug)]
    #[repr(i32)]
    pub enum KConfigAccessMode {
        NoAccess = 0,
        ReadOnly = 1,
        ReadWrite = 2,
    }

    #[namespace = "rust::kf6::kconfig"]
    #[derive(Debug)]
    #[repr(u32)]
    enum OpenFlag {
        IncludeGlobals = 1,
        CascadeConfig = 2,

        SimpleConfig = 0,
        NoCascade = 1,
        NoGlobals = 2,
        FullConfig = 3,
    }
    #[namespace = "rust::kf6::kconfig"]
    unsafe extern "C++" {
        fn from(file: QString, mode: OpenFlags) -> UniquePtr<KConfig>;

        type KConfigAccessMode;
        type OpenFlag;
        type OpenFlags = super::OpenFlags;
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
        type WriteConfigFlags = crate::kconfig::WriteConfigFlags;

        fn groupList(self: &KConfig) -> QStringList;

        fn hasGroup(self: &KConfig, group: &QString) -> bool;

        fn group(self: Pin<&mut KConfig>, group: &QString) -> KConfigGroup;

        fn deleteGroup(self: Pin<&mut KConfig>, group: &QString, flags: WriteConfigFlags);

        fn sync(self: Pin<&mut KConfig>) -> bool;

        fn markAsClean(self: Pin<&mut KConfig>);

        fn accessMode(self: &KConfig) -> KConfigAccessMode;

        fn isImmutable(self: &KConfig) -> bool;

        fn isGroupImmutable(self: &KConfig, group: &QString) -> bool;

        fn name(self: &KConfig) -> QString;

        fn isConfigWritable(self: Pin<&mut KConfig>, warnUser: bool) -> bool;

        fn copyFrom(self: &KConfig, config: &KConfig);

        fn checkUpdate(self: Pin<&mut KConfig>, id: &QString, updateFile: &QString);

        fn reparseConfiguration(self: Pin<&mut KConfig>);

        fn addConfigSources(self: Pin<&mut KConfig>, sources: &QStringList);

        // QStandardPaths::StandardLocation locationType() const;

    }

    #[namespace = "rust::kf6"]
    unsafe extern "C++" {}

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");
    }
}

bitflags! {
    #[repr(C)]
    pub struct OpenFlags: u32 {
        const IncludeGlobals = 1;
        const CascadeConfig = 2;
        const SimpleConfig = 0;
        const NoCascade = 1;
        const NoGlobals = 2;
        const FullConfig = 3;
    }
}

pub use ffi::KConfig;
pub use ffi::KConfigAccessMode;

impl KConfig {
    pub fn from(file: QString, mode: OpenFlags) -> UniquePtr<KConfig> {
        ffi::from(file, mode)
    }
}

unsafe impl ExternType for OpenFlags {
    type Id = type_id!("rust::kf6::kconfig::OpenFlags");
    type Kind = cxx::kind::Trivial;
}

#[cfg(test)]
mod tests {
    use super::*;

    use cxx_qt_lib::QString;

    #[test]
    fn test_add() {
        let mut config = KConfig::from(QString::from("kdeglobals"), OpenFlags::FullConfig);

        let mut group = config.as_mut().unwrap().group(&QString::from("General"));

        let _s = group.readEntry(&QString::from("ColorSchemeHash"), &QString::from("12345"));

        group.exists();

        group.writeEntry(&QString::from("ColorSchemeHash"), &QString::from("5"), WriteConfigFlags::Normal);
    }
}
