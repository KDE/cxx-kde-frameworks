// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: LGPL-2.0-or-later

use std::{collections::HashMap, path::PathBuf};

use cxx_qt_build::CxxQtBuilder;

pub fn link_libraries(builder: CxxQtBuilder) -> CxxQtBuilder {
    // load and link against KDE libs
    let mut include_dir = String::from("/usr/include/KF6/");
    let mut lib_dir = String::from("/usr/lib/");
    let libraries = HashMap::from([("CoreAddons", "KCoreAddons"), ("I18n", "KI18n"), ("Crash", "KCrash")]);

    if let Ok(dir) = std::env::var("KDE_INCLUDEDIR") {
        include_dir = dir;
    } else {
        println!(
            "cargo:warning=KDE_INCLUDEDIR is not defined, used default value: {}",
            include_dir
        );
    }
    if let Ok(dir) = std::env::var("KDE_LIBDIR") {
        lib_dir = dir;
    } else {
        println!(
            "cargo:warning=KDE_LIBDIR is not defined, used default value: {}",
            lib_dir
        );
    }

    println!(
        "cargo:rustc-link-search={}",
        std::fs::canonicalize(lib_dir)
            .expect("Cannot get canonical path of KDE_LIBDIR")
            .display()
    );

    for (k, _) in &libraries {
        println!("cargo:rustc-link-lib=KF6{}", k);
    }

    let kf6_include_path = PathBuf::from(include_dir)
        .canonicalize()
        .expect("Cannot get canonical path of KDE_INCLUDEDIR");

    builder.cc_builder(move |cc| {
        for (_, v) in &libraries {
            cc.include(format!("{}", kf6_include_path.join(v).display()));
        }
    })
}
