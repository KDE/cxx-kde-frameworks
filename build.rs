// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::PathBuf;

use cmake_package::find_package;
use cxx_qt_build::CxxQtBuilder;

// list of (LibraryName, [LibraryTargets])
const LIBRARIES: &[(&str, &[&str])] = &[
    ("KF6CoreAddons", &["KF6::CoreAddons"]),
    ("KF6I18n", &["KF6::I18n"]),
    ("KF6Crash", &["KF6::Crash"]),
    ("KF6IconThemes", &["KF6::IconThemes"]),
    ("KF6ConfigWidgets", &["KF6::ConfigWidgets"]),
    ("KF6KCMUtils", &["KF6::KCMUtilsQuick"]),
];

fn main() {
    write_headers();

    let mut builder = CxxQtBuilder::new()
        .include_prefix("private")
        .crate_include_root(Some("include/".to_owned()))
        .include_dir(header_dir());

    let rust_files = vec![
        "kcoreaddons/kaboutdata",
        "kcoreaddons/kformat",
        "kcoreaddons/kpluginmetadata",
        "ki18n/klocalizedcontext",
        "ki18n/klocalizedstring",
        "kcrash/kcrash",
        "kiconthemes/kicontheme",
        "kconfigwidgets/kstylemanager",
        "kcmutils/kquickconfigmodule",
    ];

    let cpp_files = vec![
        "kcoreaddons/kaboutdata",
        "kcoreaddons/kformat",
        "kcoreaddons/kpluginmetadata",
        "ki18n/klocalizedcontext",
        "ki18n/klocalizedstring",
        "kcrash/kcrash",
        "kiconthemes/kicontheme",
        "kcmutils/kquickconfigmodule",
    ];

    for file in &rust_files {
        builder = builder.file(format!("src/{file}.rs"))
    }

    for file in &cpp_files {
        builder = builder.cpp_file(format!("src/{file}.cpp"))
    }

    builder = link_libraries(builder);

    let interface = builder.build();
    interface.reexport_dependency("cxx-qt-lib").export();
}

fn write_headers() {
    println!("cargo::rerun-if-changed=include/");
    std::fs::create_dir_all(header_dir()).expect("Failed to create include directory");

    write_headers_in("kcoreaddons");
    write_headers_in("ki18n");
    write_headers_in("kcrash");
    write_headers_in("kiconthemes");
    write_headers_in("kconfigwidgets");
    write_headers_in("kcmutils");
}

fn write_headers_in(subfolder: &str) {
    println!("cargo::rerun-if-changed=include/{subfolder}");

    for entry in
        std::fs::read_dir(format!("include/{subfolder}")).expect("Failed to read include directory")
    {
        let entry = entry.expect("Failed to read header file!");
        let header_name = entry.file_name();
        println!(
            "cargo::rerun-if-changed=include/{subfolder}/{header_name}",
            header_name = header_name.to_string_lossy()
        );

        // TODO: Do we want to add the headers into a subdirectory?
        std::fs::copy(entry.path(), header_dir().join(header_name))
            .expect("Failed to copy header file!");
    }
}

fn header_dir() -> PathBuf {
    PathBuf::from(std::env::var("OUT_DIR").unwrap())
        .join("include")
        .join("cxx-kde-frameworks")
}

fn link_libraries(builder: CxxQtBuilder) -> CxxQtBuilder {
    let mut directories = Vec::new();

    for (name, targets) in LIBRARIES {
        match find_package(*name).find() {
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

    unsafe {
        builder.cc_builder(move |cc| {
            for dir in &directories {
                cc.include(dir);
            }
        })
    }
}
