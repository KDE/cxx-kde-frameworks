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

        #[rust_name = "add_author_raw"]
        fn addAuthor(
            self: Pin<&mut KAboutData>,
            name: &QString,
            task: &QString,
            email_address: &QString,
            web_address: &QString,
            avatar_url: &QUrl,
        ) -> Pin<&mut KAboutData>;

        #[rust_name = "add_credit_raw"]
        fn addCredit(
            self: Pin<&mut KAboutData>,
            name: &QString,
            task: &QString,
            email_address: &QString,
            web_address: &QString,
            avatar_url: &QUrl,
        ) -> Pin<&mut KAboutData>;

        #[rust_name = "set_translator_raw"]
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
        unsafe fn processCommandLine(
            self: Pin<&mut KAboutData>,
            parser: *mut QCommandLineParser,
        );
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
}

use std::pin::Pin;

use cxx::UniquePtr;
use cxx_qt_lib::QString;
use cxx_qt_lib_extras::QCommandLineParser;
pub use ffi::KAboutData;

use super::KAuthor;
use super::KCredit;
use super::KTranslator;
use super::License;

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

    pub fn set_application_data(about_data: UniquePtr<KAboutData>) {
        if let Some(data) = about_data.as_ref() {
            ffi::set_application_data(data);
        } else {
            eprintln!("KAboutData couldn't be unwrapped");
        }
    }

    pub fn add_author(self: Pin<&mut KAboutData>, author: KAuthor) -> Pin<&mut KAboutData> {
        return self.add_author_raw(
            &author.name,
            &author.task,
            &author.email_address,
            &author.web_address,
            &author.avatar_url,
        );
    }

    pub fn add_credit(self: Pin<&mut KAboutData>, credit: KCredit) -> Pin<&mut KAboutData> {
        return self.add_credit_raw(
            &credit.name,
            &credit.task,
            &credit.email_address,
            &credit.web_address,
            &credit.avatar_url,
        );
    }

    pub fn set_translator(
        self: Pin<&mut KAboutData>,
        translator: KTranslator,
    ) -> Pin<&mut KAboutData> {
        return self.set_translator_raw(&translator.name, &translator.email_address);
    }

    pub fn setup_command_line(
        self: Pin<&mut KAboutData>,
        parser: &mut QCommandLineParser,
    ) -> bool {
        unsafe {
            self.setup_command_line_raw(&mut *parser)
        }
    }

    pub fn process_command_line(
        self: Pin<&mut KAboutData>,
        parser: &mut QCommandLineParser,
    ) {
         unsafe {
            self.process_command_line_raw(&mut *parser);
        }
    }
}
