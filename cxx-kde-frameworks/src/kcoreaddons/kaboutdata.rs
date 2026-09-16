// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use cxx::{ExternType, type_id};
use std::mem::MaybeUninit;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;

        include!("cxx-qt-lib-extras/core/qcommandlineparser.h");
        type QCommandLineParser = cxx_qt_lib_extras::QCommandLineParser;

        include!("cxx-kde-frameworks/kcoreaddons/kaboutdata.h");
        type KAboutData;
        type KAboutPerson = super::KAboutPerson;

        /// Returns the person's name
        ///
        /// [C++ API documentation](https://api.kde.org/kaboutperson.html#name)
        fn name(self: &KAboutPerson) -> QString;

        /// Returns the person's task
        ///
        /// [C++ API documentation](https://api.kde.org/kaboutperson.html#task)
        fn task(self: &KAboutPerson) -> QString;

        /// Returns the person's email address
        ///
        /// [C++ API documentation](https://api.kde.org/kaboutperson.html#emailAddress)
        #[rust_name = "email_address"]
        fn emailAddress(self: &KAboutPerson) -> QString;

        /// Returns the person's home page
        ///
        /// [C++ API documentation](https://api.kde.org/kaboutperson.html#webAddress)
        #[rust_name = "web_address"]
        fn webAddress(self: &KAboutPerson) -> QString;

        /// Returns an URL pointing to the user's avatar
        ///
        /// [C++ API documentation](https://api.kde.org/kaboutperson.html#avatarUrl)
        #[rust_name = "avatar_url"]
        fn avatarUrl(self: &KAboutPerson) -> QUrl;

        // static KAboutPerson fromJSON(const QJsonObject &obj);
        // Needs QJsonObject

        /// Add an author.
        ///
        /// [C++ API documentation](https://api.kde.org/kaboutdata.html#addAuthor)
        #[rust_name = "add_author"]
        fn addAuthor(self: Pin<&mut KAboutData>, author: &KAboutPerson) -> Pin<&mut KAboutData>;

        /// Add a person that deserves credit.
        ///
        /// [C++ API documentation](https://api.kde.org/kaboutdata.html#addCredit)
        #[rust_name = "add_credit"]
        fn addCredit(self: Pin<&mut KAboutData>, author: &KAboutPerson) -> Pin<&mut KAboutData>;

        /// Sets the name(s) of the translator(s) of the GUI.
        ///
        /// [C++ API documentation](https://api.kde.org/kaboutdata.html#setTranslator)
        #[rust_name = "set_translator"]
        fn setTranslator(
            self: Pin<&mut KAboutData>,
            name: &QString,
            email_address: &QString,
        ) -> Pin<&mut KAboutData>;

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

    #[namespace = "rust::bridge::kaboutdata"]
    unsafe extern "C++" {

        #[doc(hidden)]
        #[rust_name = "kaboutdata_from"]
        fn constructKAboutData(
            component_name: &str,
            display_name: &str,
            version: &str,
            short_description: &str,
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
        #[rust_name = "kaboutperson_drop"]
        fn drop(format: &mut KAboutPerson);
    }

    #[namespace = "rust::bridge::kaboutperson"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "kaboutperson_from"]
        fn constructKAboutPerson(
            name: &str,
            task: &str,
            email_address: &str,
            web_address: &str,
            avatar_url: &QUrl,
        ) -> KAboutPerson;
    }
}

/// This class is used to store information about a person or developer.
///
/// [C++ API documentation](https://api.kde.org/kaboutperson.html)
#[repr(C)]
pub struct KAboutPerson {
    _cspec: MaybeUninit<usize>,
}

use std::pin::Pin;

use cxx::UniquePtr;
use cxx_qt_lib::QUrl;
use cxx_qt_lib_extras::QCommandLineParser;

use super::License;

/// This class is used to store information about a program or plugin.
///
/// [C++ API documentation](https://api.kde.org/kaboutdata.html)
pub use ffi::KAboutData;

impl KAboutData {
    /// Create a new KAboutData
    pub fn from(
        component_name: &str,
        display_name: &str,
        version: &str,
        short_description: &str,
        license: License,
    ) -> UniquePtr<KAboutData> {
        ffi::kaboutdata_from(
            component_name,
            display_name,
            version,
            short_description,
            license as i32,
        )
    }

    /// Sets the application data for this application.
    ///
    /// [C++ API documentation](https://api.kde.org/kaboutdata.html#setApplicationData)
    pub fn set_application_data(about_data: &KAboutData) {
        ffi::set_application_data(about_data);
    }

    /// Configures the parser command line parser to provide an authors entry with information about the developers of the application and an entry specifying the license.
    ///
    /// [C++ API documentation](https://api.kde.org/kaboutdata.html#setupCommandLine)
    pub fn setup_command_line(self: Pin<&mut KAboutData>, parser: &mut QCommandLineParser) -> bool {
        unsafe { self.setup_command_line_raw(&mut *parser) }
    }

    /// Reads the processed parser and sees if any of the arguments are the ones set up from setupCommandLine().
    ///
    /// [C++ API documentation](https://api.kde.org/kaboutdata.html#processCommandLine)
    pub fn process_command_line(self: Pin<&mut KAboutData>, parser: &mut QCommandLineParser) {
        unsafe {
            self.process_command_line_raw(&mut *parser);
        }
    }
}

impl KAboutPerson {
    /// Create a new KAboutPerson
    pub fn from(
        name: &str,
        task: &str,
        email_address: &str,
        web_address: &str,
        avatar_url: &QUrl,
    ) -> KAboutPerson {
        ffi::kaboutperson_from(name, task, email_address, web_address, avatar_url)
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

    use cxx_qt_lib::QUrl;

    #[test]
    fn test_add() {
        let mut about = KAboutData::from("myapp", "My App", "1.0", "An awesome app", License::GPL);

        about.pin_mut().add_author(&KAboutPerson::from(
            "Me",
            "Author",
            "me@kde.org",
            "https://kde.org",
            &QUrl::default(),
        ));

        about.pin_mut().add_credit(&KAboutPerson::from(
            "Somebody",
            "Fixes",
            "someone@kde.org",
            "https://kde.org",
            &QUrl::default(),
        ));
    }
}
