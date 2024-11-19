// SPDX-FileCopyrightText: 2024 Nicolas Fella <nicolas.fell@gmx.de>
// SPDX-License-Identifier: MPL-2.0

#[cxx_qt::bridge]
mod ffi {

    unsafe extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;

        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-kde-frameworks/udsentry.h");
        type UDSEntry = crate::kio::udsentry::UDSEntry;
    }

    unsafe extern "C++" {
        include!("cxx-kde-frameworks/workerbase.h");

        type WorkerBase;

        type WorkerResult;

        pub fn data(self: Pin<&mut WorkerBase>, data: &QByteArray);

        pub fn listEntry(self: Pin<&mut WorkerBase>, entry: &UDSEntry);

        pub fn workerResultPass() -> UniquePtr<WorkerResult>;

        pub fn workerResultFail(code: i32, message: &QString) -> UniquePtr<WorkerResult>;

    }
}

use cxx::UniquePtr;
use cxx_qt_lib::QString;

impl WorkerResult {
    pub fn pass() -> UniquePtr<WorkerResult> {
        ffi::workerResultPass()
    }

    pub fn fail(code: i32, message: &QString) -> UniquePtr<WorkerResult> {
        ffi::workerResultFail(code, message)
    }
}

pub use ffi::WorkerBase;
pub use ffi::WorkerResult;
