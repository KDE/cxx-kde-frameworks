// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fell@gmx.de>
// SPDX-License-Identifier: MPL-2.0

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {

        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = cxx_qt_lib::QStringList;

        include!("cxx-kde-frameworks/kconfiggroup.h");
        type KConfigGroup = super::KConfigGroup;

        fn readEntry(self: &KConfigGroup, key: &QString, aDefault: &QString) -> QString;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");
    }
}

use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

pub struct KConfigGroup {
    _cspec: MaybeUninit<usize>,
    _cspec2: MaybeUninit<usize>,
}

// Safety:

// Static checks on the C++ side ensure that KConfigGroup is trivial.
unsafe impl ExternType for KConfigGroup {
    type Id = type_id!("KConfigGroup");
    type Kind = cxx::kind::Trivial;
}

#[cfg(test)]
mod tests {
    use super::*;
    use cxx_qt_lib::QString;

    #[test]
    fn test() {
    }
}
