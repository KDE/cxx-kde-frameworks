// SPDX-FileCopyrightText: 2024 Nicolas Fella <nicolas.fell@gmx.de>
// SPDX-License-Identifier: MPL-2.0

use cxx::{type_id, ExternType};
use cxx_qt_lib::QString;
use std::mem::MaybeUninit;

#[cxx_qt::bridge()]
mod ffi {

    /// This enum chooses what dialect is used for binary units.
    ///
    /// [C++ API documentation](https://api-staging.kde.org/kformat.html#BinaryUnitDialect-enum)
    #[namespace = "rust::kf6"]
    #[repr(i32)]
    pub enum KFormatBinaryUnitDialect {
        DefaultBinaryDialect = -1,
        IECBinaryDialect = 0,
        JEDECBinaryDialect = 1,
        MetricBinaryDialect = 2,
    }

    /// This enum chooses what dialect is used for binary units.
    ///
    /// [C++ API documentation](https://api-staging.kde.org/kformat.html#BinarySizeUnits-enum)
    #[namespace = "rust::kf6"]
    #[repr(i32)]
    pub enum KFormatBinarySizeUnits {
        DefaultBinaryUnits = -1,
        UnitByte,
        UnitKiloByte,
        UnitMegaByte,
        UnitGigaByte,
        UnitTeraByte,
        UnitPetaByte,
        UnitExaByte,
        UnitZettaByte,
        UnitYottaByte,
    }

    /// These prefixes are used in KDE by the formatValue() function.
    ///
    /// [C++ API documentation](https://api-staging.kde.org/kformat.html#UnitPrefix-enum)
    #[namespace = "rust::kf6"]
    #[repr(i32)]
    enum KFormatUnitPrefix {
        AutoAdjust = -128,
        Yocto = 0,
        Zepto,
        Atto,
        Femto,
        Pico,
        Nano,
        Micro,
        Milli,
        Centi,
        Deci,
        Unity,
        Deca,
        Hecto,
        Kilo,
        Mega,
        Giga,
        Tera,
        Peta,
        Exa,
        Zetta,
        Yotta,
    }

    #[namespace = "rust::kf6"]
    unsafe extern "C++" {
        #[rust_name = "format_spellout_duration"]
        fn formatSpelloutDuration(fmt: &KFormat, msecs: u64) -> QString;

        #[rust_name = "format_decimal_duration"]
        fn formatDecimalDuration(fmt: &KFormat, msecs: u64, decimalPlaces: i32) -> QString;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-kde-frameworks/kformat.h");
        type KFormat = super::KFormat;

        // include!("cxx-qt-lib/qdate.h");
        // type QDate = cxx_qt_lib::QDate;

        /// Converts size from bytes to the appropriate string representation using the binary unit dialect dialect and the specific units units.
        ///
        /// [C++ API documentation](https://api-staging.kde.org/kformat.html#formatByteSize)
        #[rust_name = "format_byte_size"]
        fn formatByteSize(
            self: &KFormat,
            size: f64,
            precision: i32,
            dialect: KFormatBinaryUnitDialect,
            units: KFormatBinarySizeUnits,
        ) -> QString;

        // TODO needs QLocale
        // QString formatRelativeDate(const QDate &date, QLocale::FormatType format) const;
        // #[rust_name = "format_relative_date"]
        // fn formatRelativeDate(self: &KFormat, date: &QDate, format: QLocaleFormatType) -> QString;

        /// Converts value to the appropriate string representation.
        ///
        /// [C++ API documentation](https://api-staging.kde.org/kformat.html#formatValue)
        #[rust_name = "format_value"]
        fn formatValue(
            self: &KFormat,
            value: f64,
            unit: &QString,
            precision: i32,
            prefix: KFormatUnitPrefix,
            dialect: KFormatBinaryUnitDialect,
        ) -> QString;

    }

    #[namespace = "rust::kf6"]
    unsafe extern "C++" {
        type KFormatBinaryUnitDialect;
        type KFormatBinarySizeUnits;
        type KFormatUnitPrefix;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "kformat_init_default"]
        fn construct() -> KFormat;

        #[doc(hidden)]
        #[rust_name = "kformat_drop"]
        fn drop(format: &mut KFormat);
    }
}

pub use ffi::KFormatBinarySizeUnits;
pub use ffi::KFormatBinaryUnitDialect;
pub use ffi::KFormatUnitPrefix;

/// Class for formatting numbers and datetimes.
///
/// [C++ API documentation](https://api-staging.kde.org/kformat.html)
#[repr(C)]
pub struct KFormat {
    _cspec: MaybeUninit<usize>,
}

impl Default for KFormat {
    fn default() -> Self {
        ffi::kformat_init_default()
    }
}

impl Drop for KFormat {
    fn drop(&mut self) {
        ffi::kformat_drop(self);
    }
}

impl KFormat {

    /// Given a number of milliseconds, converts that to a spell-out string containing the localized equivalent.
    ///
    /// [C++ API documentation](https://api-staging.kde.org/kformat.html#formatSpelloutDuration)
    pub fn format_spellout_duration(&self, msecs: u64) -> QString {
        ffi::format_spellout_duration(self, msecs)
    }

    /// Given a number of milliseconds, converts that to a string containing the localized equivalent to the requested decimal places.
    ///
    /// [C++ API documentation](https://api-staging.kde.org/kformat.html#formatDecimalDuration)
    pub fn format_decimal_duration(&self, msecs: u64, decimal_places: i32) -> QString {
        ffi::format_decimal_duration(self, msecs, decimal_places)
    }
}

// Safety:

// Static checks on the C++ side ensure that KFormat is trivial.
unsafe impl ExternType for KFormat {
    type Id = type_id!("KFormat");
    type Kind = cxx::kind::Trivial;
}

#[cfg(test)]
mod tests {
    use super::*;

    use cxx_qt_lib::QString;

    #[test]
    fn test_add() {
        std::env::set_var("LC_NUMERIC", "en_US.UTF-8");

        let fm = KFormat::default();

        assert_eq!(
            fm.format_spellout_duration(1234),
            QString::from("1 second(s)")
        );

        assert_eq!(
            fm.format_decimal_duration(1234, 2),
            QString::from("1.23 seconds")
        );

        assert_eq!(
            fm.format_byte_size(
                55.0,
                2,
                KFormatBinaryUnitDialect::DefaultBinaryDialect,
                KFormatBinarySizeUnits::UnitKiloByte
            ),
            QString::from("0.05 KiB")
        );

        // assert_eq!(
        //     fm.format_relative_date(
        //         &QDate::from_string(&QString::from("31.12.2004"), &QString::from("dd.MM.yyyy"))
        //             .unwrap(),
        //         QLocaleFormatType::LongFormat
        //     ),
        //     QString::from("Freitag, 31. Dezember 2004")
        // );
    }
}
