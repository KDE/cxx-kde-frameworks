// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0
mod build_files;

use build_files::{CPP_FILES, LIBRARIES, RUST_BRIDGES};
use cmake_package;
use cxx_qt_build::CxxQtBuilder;

fn main() {
    let mut builder = CxxQtBuilder::new()
        .crate_include_root(Some("src/".to_owned()))
        .files(RUST_BRIDGES)
        .cpp_files(CPP_FILES);
    
    let mut include_dirs = Vec::new();

    let type_lib_include = std::env::var("DEP_QTBRIDGE_TYPE_LIB_INCLUDE")
    .expect("DEP_QTBRIDGE_TYPE_LIB_INCLUDE not set. This variable should have been set by qtbridge-type-lib");
    include_dirs.push(type_lib_include);
    
    include_dirs.extend(link_kf_libraries());
    
    unsafe {
        builder = builder.cc_builder(move |cc| {
            cc.includes(include_dirs.clone());
        });
    }

    let interface = builder.build();
    interface.reexport_dependency("cxx-qt-lib").export();
}

fn link_kf_libraries() -> Vec<String> {
    let mut include_dirs = Vec::new();

    for (name, targets) in LIBRARIES {
        match cmake_package::find_package(*name).find() {
            Err(err) => panic!("Cannot find {name}: {err:?}"),
            Ok(package) => {
                for target in *targets {
                    let cmake_target = package.target(target.to_owned()).unwrap();
                    cmake_target.link();
                    include_dirs.extend(cmake_target.include_directories);
                }
            }
        }
    }

    // hack for qqmlintegration.h
    match cmake_package::find_package("Qt6")
        .components(["QmlIntegration".into()])
        .find()
    {
        Err(err) => panic!("Cannot find Qt: {err:?}"),
        Ok(package) => match package.target("Qt6::QmlIntegration") {
            Some(target) => include_dirs.extend(target.include_directories),
            None => panic!("Couldn't find QmlIntegration"),
        },
    }

    include_dirs
}
