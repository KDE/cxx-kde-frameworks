// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use cxx_qt_lib::{QString, QUrl};

pub struct KAuthor {
    pub name: QString,
    pub task: QString,
    pub email_address: QString,
    pub web_address: QString,
    pub avatar_url: QUrl,
}

impl Default for KAuthor {
    fn default() -> Self {
        return Self {
            name: QString::from("Author"),
            task: QString::from(""),
            email_address: QString::from(""),
            web_address: QString::from(""),
            avatar_url: QUrl::from(""),
        };
    }
}

pub struct KCredit {
    pub name: QString,
    pub task: QString,
    pub email_address: QString,
    pub web_address: QString,
    pub avatar_url: QUrl,
}

impl Default for KCredit {
    fn default() -> Self {
        return Self {
            name: QString::from("Credit"),
            task: QString::from(""),
            email_address: QString::from(""),
            web_address: QString::from(""),
            avatar_url: QUrl::from(""),
        };
    }
}

pub struct KTranslator {
    pub name: QString,
    pub email_address: QString,
}

impl Default for KTranslator {
    fn default() -> Self {
        return Self {
            name: QString::from("Translator"),
            email_address: QString::from(""),
        };
    }
}
