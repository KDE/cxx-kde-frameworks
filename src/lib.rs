#[cfg(not(any(cxxqt_qt_version_major = "5", cxxqt_qt_version_major = "6")))]
compile_error!("cxxqt_qt_version_major must be either \"5\" or \"6\"");

pub mod kcoreaddons;
pub mod ki18n;
pub mod kcrash;

#[cfg(test)]
mod tests {

    use super::*;

    use kcoreaddons::KFormat;
    use kcoreaddons::KFormatBinarySizeUnits;
    use kcoreaddons::KFormatBinaryUnitDialect;

    use cxx_qt_lib::QString;
    // use cxx_qt_lib::QDate;

    #[test]
    fn test_add() {
        let fm = KFormat::default();

        // assert_eq!(
        //     fm.format_spellout_duration(1234),
        //     QString::from("1 second(s)")
        // );

        // assert_eq!(
        //     fm.format_decimal_duration(1234, 2),
        //     QString::from("1,23 seconds")
        // );

        assert_eq!(
            fm.format_byte_size(
                55.0,
                2,
                KFormatBinaryUnitDialect::DefaultBinaryDialect,
                KFormatBinarySizeUnits::UnitKiloByte
            ),
            QString::from("0,05 KiB")
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
