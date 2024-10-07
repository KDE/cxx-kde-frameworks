mod kf6_build;
use std::path::PathBuf;

use cxx_qt_build::CxxQtBuilder;

fn header_dir() -> PathBuf {
    PathBuf::from(std::env::var("OUT_DIR").unwrap())
        .join("include")
        .join("cxx-kde-frameworks")
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

fn write_headers() {
    println!("cargo::rerun-if-changed=include/");
    std::fs::create_dir_all(header_dir()).expect("Failed to create include directory");

    write_headers_in("kcoreaddons");
    write_headers_in("ki18n");
    write_headers_in("kcrash");
}

fn main() {
    write_headers();

    let interface = cxx_qt_build::Interface::default()
        .export_include_prefixes([])
        .export_include_directory(header_dir(), "cxx-kde-frameworks")
        .reexport_dependency("cxx-qt-lib");

    let mut builder = CxxQtBuilder::library(interface);
    builder = kf6_build::link_libraries(builder);

    let rust_files = vec![
        "kcoreaddons/kaboutdata",
        "kcoreaddons/kformat",
        "ki18n/klocalizedcontext",
        "ki18n/klocalizedstring",
        "kcrash/kcrash"
    ];

    for source in &rust_files {
        builder = builder.file(format!("src/{source}.rs"))
    }

    let cpp_files = vec![
        "kcoreaddons/kaboutdata",
        "kcoreaddons/kformat",
        "ki18n/klocalizedcontext",
        "ki18n/klocalizedstring",
        "kcrash/kcrash"
    ];

    builder = builder.cc_builder(move |cc| {
        for file in &cpp_files {
            cc.file(format!("src/{file}.cpp"));
            println!("cargo:rerun-if-changed=src/{file}.cpp");
        }
    });

    builder.build();
}
