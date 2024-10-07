// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::PathBuf;

use cmake_package::find_package;
use cxx_qt_build::CxxQtBuilder;

// list of (LibraryName, [LibraryTargets])
const LIBRARIES: [(&str, &[&str]); 3] = [
    ("KF6CoreAddons", &["KF6::CoreAddons"]),
    ("KF6I18n", &["KF6::I18n"]),
    ("KF6Crash", &["KF6::Crash"]),
];

fn main() {
    write_headers();

    let interface = cxx_qt_build::Interface::default()
        .export_include_prefixes([])
        .export_include_directory(header_dir(), "cxx-kde-frameworks")
        .reexport_dependency("cxx-qt-lib");

    let mut builder = CxxQtBuilder::library(interface);
    builder = setup_linker(builder);

    let rust_files = vec![
        "kcoreaddons/kaboutdata",
        "kcoreaddons/kformat",
        "ki18n/klocalizedcontext",
        "ki18n/klocalizedstring",
        "kcrash/kcrash",
    ];

    for source in &rust_files {
        builder = builder.file(format!("src/{source}.rs"))
    }

    let cpp_files = vec![
        "kcoreaddons/kaboutdata",
        "kcoreaddons/kformat",
        "ki18n/klocalizedcontext",
        "ki18n/klocalizedstring",
        "kcrash/kcrash",
    ];

    builder = builder.cc_builder(move |cc| {
        for file in &cpp_files {
            cc.file(format!("src/{file}.cpp"));
            println!("cargo:rerun-if-changed=src/{file}.cpp");
        }
    });

    builder.build();
}

fn write_headers() {
    println!("cargo::rerun-if-changed=include/");
    std::fs::create_dir_all(header_dir()).expect("Failed to create include directory");

    write_headers_in("kcoreaddons");
    write_headers_in("ki18n");
    write_headers_in("kcrash");
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

fn setup_linker(builder: CxxQtBuilder) -> CxxQtBuilder {
    let mut directories = Vec::new();

    for (name, targets) in LIBRARIES {
        match find_package(name).find() {
            Err(_) => panic!("Cannot find {name}"),
            Ok(package) => {
                for target in targets {
                    let cmake_target = package.target(target.to_owned()).unwrap();
                    cmake_target.link();
                    for dir in cmake_target.include_directories {
                        directories.push(dir);
                    }
                }
            }
        }
    }

    builder.cc_builder(move |cc| {
        for dir in &directories {
            cc.include(dir);
        }
    })
}
