/**
 * SPDX-FileCopyrightText: (C) 2026 Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
 *
 * SPDX-License-Identifier: MPL-2.0
 */

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import org.kde.ki18n
import org.kde.kirigami as Kirigami
import org.kde.kirigamiaddons.formcard as FormCard

Kirigami.ApplicationWindow {
    id: root
    width: 400
    height: 400

    visible: true
    title: KI18n.i18n("kapp")

    Component {
           id: about
           FormCard.AboutPage {}
    }

    pageStack.initialPage: Kirigami.Page {
        ColumnLayout {
            anchors.fill: parent
            
            Label {
                font.pixelSize: 20
                text: KI18n.i18n("KDE ❤️ Rust")
                Layout.alignment: Qt.AlignHCenter 
            }

            Item {
                implicitHeight: Kirigami.Units.largeSpacing
            }

            Button {
                text: "About"
                onClicked: root.pageStack.push(about)
                Layout.alignment: Qt.AlignHCenter 
            }
        }
    }

}
