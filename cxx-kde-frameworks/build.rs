// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use cmake_package;
use qtbridge_build_utils::qt_build::QtInstallation;
use std::path::Path;

// list of (LibraryName, [LibraryTargets])
const LIBRARIES: &[(&'static str, &'static [&'static str])] = &[
    // ("KF6CoreAddons", &["KF6::CoreAddons"]),
    ("KF6I18n", &["KF6::I18n", "KF6::I18nQml"]),
    ("KF6Crash", &["KF6::Crash"]),
    ("KF6IconThemes", &["KF6::IconThemes"]),
    ("KF6ConfigWidgets", &["KF6::ConfigWidgets"]),
    // ("KF6KCMUtils", &["KF6::KCMUtilsQuick"]),
];

const RUST_BRIDGES: &[&str] = &[
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

const CPP_FILES: &[&str] = &[
    // "src/kcoreaddons/kaboutdata",
    // "src/kcoreaddons/kformat",
    // "src/kcoreaddons/kpluginmetadata",
    "src/ki18n/klocalization.cpp",
    "src/ki18n/klocalizedstring.cpp",
    "src/kcrash/kcrash.cpp",
    "src/kiconthemes/kicontheme.cpp",
    // "src/kcmutils/kquickconfigmodule",
];

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let include_path = std::path::Path::new(&manifest_dir).join("src");

    // This becomes DEP_QTBRIDGE_TYPE_LIB_INCLUDE in dependents
    println!("cargo:include={}", include_path.display());
    println!("cargo::metadata=include={}", include_path.display());

    let qt = QtInstallation::default();
    for file in CPP_FILES {
        println!("cargo::rerun-if-changed={file}");
    }
    let mut builder = cxx_build::bridges(RUST_BRIDGES);
    builder
        .std("c++17")
        .flag_if_supported("/Zc:__cplusplus")
        .flag_if_supported("/permissive-")
        .include("src")
        .include("../");
    qt.configure_builder(&mut builder);

    CPP_FILES.iter().for_each(|file| {
        builder.file(file);
        println!("cargo::rerun-if-changed={file}");
        let h_path = Path::new(file).with_extension("").with_extension("h");
        if h_path.is_file() {
            println!("cargo::rerun-if-changed={}", h_path.to_str().unwrap());
        }
    });

    let qt_modules = ["Core", "Gui", "Qml", "QmlIntegration"];
    for include_dir in qt.include_dirs(qt_modules, true) {
        builder.include(include_dir);
    }
    let qt_modules = ["Core", "Gui", "Qml"];
    qt.link_modules(qt_modules);
    link_libraries(&mut builder);

    builder.compile("cxx-kde-frameworks");
}

fn link_libraries(builder: &mut cc::Build) {
    let mut directories = Vec::new();

    for (name, targets) in LIBRARIES {
        match cmake_package::find_package(*name).find() {
            Err(err) => panic!("Cannot find {name}: {err:?}"),
            Ok(package) => {
                for target in *targets {
                    let cmake_target = package.target(target.to_owned()).unwrap();
                    cmake_target.link();
                    for dir in cmake_target.include_directories {
                        directories.push(dir);
                    }
                }
            }
        }
    }

    for dir in &directories {
        builder.include(dir);
    }
}
