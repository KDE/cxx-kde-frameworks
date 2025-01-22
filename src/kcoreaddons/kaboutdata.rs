// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use cxx::{type_id, ExternType};
use ffi::kaboutperson_init;
use std::mem::MaybeUninit;

#[cxx_qt::bridge]
mod ffi {

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;

        include!("cxx-qt-lib-extras/qcommandlineparser.h");
        type QCommandLineParser = cxx_qt_lib_extras::QCommandLineParser;

        include!("cxx-kde-frameworks/kaboutdata.h");
        type KAboutData;
        type KAboutPerson = super::KAboutPerson;


        fn name(self: &KAboutPerson) -> QString;

        fn task(self: &KAboutPerson) -> QString;

        #[rust_name = "email_address"]
        fn emailAddress(self: &KAboutPerson) -> QString;

        #[rust_name = "web_address"]
        fn webAddress(self: &KAboutPerson) -> QString;

        #[rust_name = "avatar_url"]
        fn avatarUrl(self: &KAboutPerson) -> QUrl;

        // static KAboutPerson fromJSON(const QJsonObject &obj);
        // Needs QJsonObject

        #[rust_name = "add_author"]
        fn addAuthor(self: Pin<&mut KAboutData>, author: &KAboutPerson) -> Pin<&mut KAboutData>;

        #[rust_name = "add_credit"]
        fn addCredit(self: Pin<&mut KAboutData>, author: &KAboutPerson) -> Pin<&mut KAboutData>;

        #[rust_name = "set_translator"]
        fn setTranslator(self: Pin<&mut KAboutData>, name: &QString, email_address: &QString) -> Pin<&mut KAboutData>;

        #[doc(hidden)]
        #[rust_name = "setup_command_line_raw"]
        unsafe fn setupCommandLine(
            self: Pin<&mut KAboutData>,
            parser: *mut QCommandLineParser,
        ) -> bool;

        #[doc(hidden)]
        #[rust_name = "process_command_line_raw"]
        unsafe fn processCommandLine(self: Pin<&mut KAboutData>, parser: *mut QCommandLineParser);
    }

    #[namespace = "rust::kf6"]
    unsafe extern "C++" {
        fn from(
            component_name: QString,
            display_name: QString,
            version: QString,
            short_description: QString,
            license: i32,
        ) -> UniquePtr<KAboutData>;

        #[rust_name = "set_application_data"]
        fn setApplicationData(about_data: &KAboutData);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "kaboutperson_init_default"]
        fn construct() -> KAboutPerson;

        #[doc(hidden)]
        #[rust_name = "kaboutperson_init"]
        fn construct(
            name: &QString,
            task: &QString,
            email_address: &QString,
            web_address: &QString,
            avatar_url: &QUrl,
        ) -> KAboutPerson;

        #[doc(hidden)]
        #[rust_name = "kaboutperson_drop"]
        fn drop(format: &mut KAboutPerson);
    }
}

#[repr(C)]
pub struct KAboutPerson {
    _cspec: MaybeUninit<usize>,
}

use std::pin::Pin;

use cxx::UniquePtr;
use cxx_qt_lib::QString;
use cxx_qt_lib::QUrl;
use cxx_qt_lib_extras::QCommandLineParser;

use super::License;

/// This class is used to store information about a program or plugin.
/// [C++ API](https://api.kde.org/frameworks/kcoreaddons/html/classKAboutData.html)
pub use ffi::KAboutData;

impl KAboutData {
    pub fn from(
        component_name: QString,
        display_name: QString,
        version: QString,
        short_description: QString,
        license: License,
    ) -> UniquePtr<KAboutData> {
        ffi::from(
            component_name,
            display_name,
            version,
            short_description,
            license as i32,
        )
    }

    pub fn set_application_data(about_data: &KAboutData) {
        ffi::set_application_data(about_data);
    }

    pub fn setup_command_line(self: Pin<&mut KAboutData>, parser: &mut QCommandLineParser) -> bool {
        unsafe { self.setup_command_line_raw(&mut *parser) }
    }

    pub fn process_command_line(self: Pin<&mut KAboutData>, parser: &mut QCommandLineParser) {
        unsafe {
            self.process_command_line_raw(&mut *parser);
        }
    }
}

impl KAboutPerson {
    pub fn from(
        name: &QString,
        task: &QString,
        email_address: &QString,
        web_address: &QString,
        avatar_url: &QUrl,
    ) -> KAboutPerson {
        kaboutperson_init(name, task, email_address, web_address, avatar_url)
    }
}
impl Default for KAboutPerson {
    fn default() -> Self {
        ffi::kaboutperson_init_default()
    }
}

impl Drop for KAboutPerson {
    fn drop(&mut self) {
        ffi::kaboutperson_drop(self);
    }
}

// Safety:

// Static checks on the C++ side ensure that KAboutPerson is trivial.
unsafe impl ExternType for KAboutPerson {
    type Id = type_id!("KAboutPerson");
    type Kind = cxx::kind::Trivial;
}

#[cfg(test)]
mod tests {
    use super::*;

    use cxx_qt_lib::QString;
    use cxx_qt_lib::QUrl;

    #[test]
    fn test_add() {
        let mut about = KAboutData::from(QString::from("myapp"), QString::from("My App"), QString::from("1.0"), QString::from("An awesome app"), License::GPL);

        about.pin_mut().add_author(&KAboutPerson::from(&QString::from("Me"), &QString::from("Author"), &QString::from("me@kde.org"), &QString::from("https://kde.org"), &QUrl::default()));

        about.pin_mut().add_credit(&KAboutPerson::from(&QString::from("Somebody"), &QString::from("Fixes"), &QString::from("someone@kde.org"), &QString::from("https://kde.org"), &QUrl::default()));
    }
}
