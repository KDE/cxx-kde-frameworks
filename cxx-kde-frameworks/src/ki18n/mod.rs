// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

mod klocalization;
mod klocalizedstring;
mod kstringextensions;

pub use klocalization::setup_localized_context;
pub use klocalizedstring::KLocalizedString;
pub use kstringextensions::*;
