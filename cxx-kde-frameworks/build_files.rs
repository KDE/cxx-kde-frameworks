// list of (LibraryName, [LibraryTargets])
pub const LIBRARIES: &[(&'static str, &'static [&'static str])] = &[
    // ("KF6CoreAddons", &["KF6::CoreAddons"]),
    ("KF6I18n", &["KF6::I18n", "KF6::I18nQml"]),
    ("KF6Crash", &["KF6::Crash"]),
    ("KF6IconThemes", &["KF6::IconThemes"]),
    ("KF6ConfigWidgets", &["KF6::ConfigWidgets"]),
    // ("KF6KCMUtils", &["KF6::KCMUtilsQuick"]),
];

pub const RUST_BRIDGES: &[&str] = &[
    // "src/kcoreaddons/kaboutdata",
    // "src/kcoreaddons/kformat",
    // "src/kcoreaddons/kpluginmetadata",
    "src/ki18n/klocalization.rs",
    "src/ki18n/klocalizedstring.rs",
    "src/kcrash/kcrash.rs",
    "src/kiconthemes/kicontheme.rs",
    "src/kconfigwidgets/kstylemanager.rs",
    // "src/kcmutils/kquickconfigmodule",
];

pub const CPP_FILES: &[&str] = &[
    // "src/kcoreaddons/kaboutdata",
    // "src/kcoreaddons/kformat",
    // "src/kcoreaddons/kpluginmetadata",
    "src/ki18n/klocalization.cpp",
    "src/ki18n/klocalizedstring.cpp",
    "src/kcrash/kcrash.cpp",
    "src/kiconthemes/kicontheme.cpp",
    // "src/kcmutils/kquickconfigmodule",
];
