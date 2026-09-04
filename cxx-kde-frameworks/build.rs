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

    builder = link_kf_libraries(builder);

    let interface = builder.build();
    interface.reexport_dependency("cxx-qt-lib").export();
}

fn link_kf_libraries(builder: CxxQtBuilder) -> CxxQtBuilder {
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

    // hack for qqmlintegration.h
    match cmake_package::find_package("Qt6").components(["QmlIntegration".into()]).find() {
        Err(err) => panic!("Cannot find Qt: {err:?}"),
        Ok(package) => {
            let res = package.target("Qt6::QmlIntegration");
            if let Some(target) = res {
                for dir in target.include_directories {
                    directories.push(dir);
                }
            } else {
                panic!("Couldn't find QmlIntegration {:?}", res);
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
