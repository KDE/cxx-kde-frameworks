/**
 * SPDX-FileCopyrightText: (C) 2026 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
 *
 * SPDX-License-Identifier: MPL-2.0
 */

import QtQuick
import QtQuick.Controls
import org.kde.ki18n

ApplicationWindow {

    visible: true
    title: KI18n.i18n("kapp")

    Label {
        font.pixelSize: 20
        text: KI18n.i18n("Hello World!")
        anchors.centerIn : parent
    }
}
