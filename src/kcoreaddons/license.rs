// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0

#[allow(non_camel_case_types)]
pub enum License {
    Custom = -2,
    File = -1,
    Unknown = 0,
    GPL = 1,
    LGPL = 2,
    BSDL = 3,
    Artistic = 4,
    GPL_V3 = 5,
    LGPL_V3 = 6,
    LGPL_V2_1 = 7,
    MIT = 8,
}

impl License {
    pub const GPL_V2: Self = Self::GPL;
    pub const LGPL_V2: Self = Self::LGPL;
}
