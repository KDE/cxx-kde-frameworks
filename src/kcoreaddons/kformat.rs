use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[cxx_qt::bridge()]
mod ffi {

    #[namespace = "rust::kf6"]
    #[repr(i32)]
    pub enum KFormatBinaryUnitDialect {
        DefaultBinaryDialect = -1,
        IECBinaryDialect = 0,
        JEDECBinaryDialect = 1,
        MetricBinaryDialect = 2,
    }

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

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-kde-frameworks/kformat.h");
        type KFormat = super::KFormat;

        // include!("cxx-qt-lib/qdate.h");
        // type QDate = cxx_qt_lib::QDate;

        // TODO u64 doesn't work
        // #[rust_name = "format_spellout_duration"]
        // fn formatSpelloutDuration(self: &KFormat, msecs: u64) -> QString;

        // TODO u64 doesn't work
        // QString formatDecimalDuration(quint64 msecs, int decimalPlaces = 2) const;
        // #[rust_name = "format_decimal_duration"]
        // fn formatDecimalDuration(self: &KFormat, msecs: u64, decimalPlaces: i32) -> QString;

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

        // QString formatValue(double value, const QString &unit, int precision, KFormat::UnitPrefix prefix, KFormat::BinaryUnitDialect dialect) const;
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
    }
}

pub use ffi::KFormatBinarySizeUnits;
pub use ffi::KFormatBinaryUnitDialect;
pub use ffi::KFormatUnitPrefix;

#[repr(C)]
pub struct KFormat {
    _cspec: MaybeUninit<usize>,
}

impl Default for KFormat {
    fn default() -> Self {
        ffi::kformat_init_default()
    }
}

// Safety:

// Static checks on the C++ side ensure that QSize is trivial.
unsafe impl ExternType for KFormat {
    type Id = type_id!("KFormat");
    type Kind = cxx::kind::Trivial;
}
