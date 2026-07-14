// SPDX-FileCopyrightText: 2024 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
// SPDX-License-Identifier: MPL-2.0


#include "cxx-kde-frameworks/src/ki18n/klocalization.h"

namespace rust::bridge::klocalization {

void inlineCppFn_setupLocalizedContext(QQmlApplicationEngine &engine)
{
    KLocalization::setupLocalizedContext(&engine);
}


} // namespace rust::bridge::klocalization
