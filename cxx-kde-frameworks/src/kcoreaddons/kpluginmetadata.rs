// SPDX-FileCopyrightText: 2024 Nicolas Fella <nicolas.fell@gmx.de>
// SPDX-License-Identifier: MPL-2.0

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {

        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = cxx_qt_lib::QStringList;

        include!("cxx-kde-frameworks/kpluginmetadata.h");
        type KPluginMetaData = super::KPluginMetaData;

        /// Returns whether this object holds valid information about a plugin.
        ///
        /// [C++ API documentation](https://api.kde.org/kpluginmetadata.html#isValid)
        #[rust_name = "is_valid"]
        fn isValid(self: &KPluginMetaData) -> bool;

        /// Returns whether this object should be hidden.
        ///
        /// [C++ API documentation](https://api.kde.org/kpluginmetadata.html#isHidden)
        #[rust_name = "is_hidden"]
        fn isHidden(self: &KPluginMetaData) -> bool;

        /// Returns the path to the plugin.
        ///
        /// [C++ API documentation](https://api.kde.org/kpluginmetadata.html#fileName)
        #[rust_name = "file_name"]
        fn fileName(self: &KPluginMetaData) -> QString;

        // QJsonObject rawData() const;

        /// Returns the user visible name of the plugin.
        ///
        /// [C++ API documentation](https://api.kde.org/kpluginmetadata.html#name)
        fn name(self: &KPluginMetaData) -> QString;

        /// Returns a short description of the plugin.
        ///
        /// [C++ API documentation](https://api.kde.org/kpluginmetadata.html#description)
        fn description(self: &KPluginMetaData) -> QString;

        // QList<KAboutPerson> authors() const;

        // QList<KAboutPerson> translators() const;

        // QList<KAboutPerson> otherContributors() const;

        /// Returns the categories of this plugin (e.g. "playlist/skin").
        ///
        /// [C++ API documentation](https://api.kde.org/kpluginmetadata.html#category)
        fn category(self: &KPluginMetaData) -> QString;

        /// Returns the icon name for this plugin.
        ///
        /// [C++ API documentation](https://api.kde.org/kpluginmetadata.html#iconName)
        #[rust_name = "icon_name"]
        fn iconName(self: &KPluginMetaData) -> QString;

        /// Returns the short license identifier (e.g. LGPL).
        ///
        /// [C++ API documentation](https://api.kde.org/kpluginmetadata.html#license)
        fn license(self: &KPluginMetaData) -> QString;

        /// Returns the text of the license, equivalent to KAboutLicense::byKeyword(license()).text()
        ///
        /// [C++ API documentation](https://api.kde.org/kpluginmetadata.html#licenseText)
        #[rust_name = "license_text"]
        fn licenseText(self: &KPluginMetaData) -> QString;

        /// Returns a short copyright statement.
        ///
        /// [C++ API documentation](https://api.kde.org/kpluginmetadata.html#copyrightText)
        #[rust_name = "copyright_text"]
        fn copyrightText(self: &KPluginMetaData) -> QString;

        /// Returns the unique identifier within the namespace of the plugin.
        ///
        /// [C++ API documentation](https://api.kde.org/kpluginmetadata.html#pluginId)
        #[rust_name = "plugin_id"]
        fn pluginId(self: &KPluginMetaData) -> QString;

        /// Returns the version of the plugin.
        ///
        /// [C++ API documentation](https://api.kde.org/kpluginmetadata.html#version)
        fn version(self: &KPluginMetaData) -> QString;

        /// Returns the website of the plugin.
        ///
        /// [C++ API documentation](https://api.kde.org/kpluginmetadata.html#website)
        fn website(self: &KPluginMetaData) -> QString;

        /// Returns the website where people can report a bug found in this plugin.
        ///
        /// [C++ API documentation](https://api.kde.org/kpluginmetadata.html#bugReportUrl)
        #[rust_name = "bug_report_url"]
        fn bugReportUrl(self: &KPluginMetaData) -> QString;

        /// Returns a list of MIME types this plugin can handle (e.g. "application/pdf", "image/png", etc.).
        ///
        /// [C++ API documentation](https://api.kde.org/kpluginmetadata.html#mimeTypes)
        #[rust_name = "mime_types"]
        fn mimeTypes(self: &KPluginMetaData) -> QStringList;

        /// Returns true if this plugin can handle the given mimetype This is more accurate than mimeTypes().contains(mimeType) because it also takes MIME type inheritance into account.
        ///
        /// [C++ API documentation](https://api.kde.org/kpluginmetadata.html#supportsMimeType)
        #[rust_name = "supports_mime_type"]
        fn supportsMimeType(self: &KPluginMetaData, mime_type: &QString) -> bool;

        /// Returns A string list of formfactors this plugin is useful for, e.g. desktop, handset or mediacenter. The keys for this are not formally defined, though the above-mentioned values should be used when applicable.
        ///
        /// [C++ API documentation](https://api.kde.org/kpluginmetadata.html#formFactors)
        #[rust_name = "form_factors"]
        fn formFactors(self: &KPluginMetaData) -> QStringList;

        /// Returns whether the plugin should be enabled by default. This is only a recommendation, applications can ignore this value if they want to.
        ///
        /// [C++ API documentation](https://api.kde.org/kpluginmetadata.html#isEnabledByDefault)
        #[rust_name = "is_enabled_by_default"]
        fn isEnabledByDefault(self: &KPluginMetaData) -> bool;

        // template<typename T>
        // bool isEnabled(const T &config) const
        // {
        //     Q_ASSERT(config.isValid());
        //     return config.readEntry(pluginId() + QLatin1String("Enabled"), isEnabledByDefault());
        // }

        // TODO make key QStringView
        /// Returns the string value for key from the metadata or defaultValue if the key does not exist
        ///
        /// [C++ API documentation](https://api.kde.org/kpluginmetadata.html#value)
        fn value(self: &KPluginMetaData, key: &QString, default_value: &QString) -> QString;

        // bool value(QStringView key, bool defaultValue) const;

        // int value(QStringView key, int defaultValue) const;

        // QStringList value(QStringView key, const QStringList &defaultValue) const;

        /// Returns true if the instance represents a static plugin
        ///
        /// [C++ API documentation](https://api.kde.org/kpluginmetadata.html#isStaticPlugin)
        #[rust_name = "is_static_plugin"]
        fn isStaticPlugin(self: &KPluginMetaData) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "kpluginmetadata_init_default"]
        fn construct() -> KPluginMetaData;

        #[doc(hidden)]
        #[rust_name = "kpluginmetadata_drop"]
        fn drop(format: &mut KPluginMetaData);
    }
}

use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

/// This class allows easily accessing some standardized values from the JSON metadata that can be embedded into Qt plugins.
///
/// [C++ API documentation](https://api.kde.org/kpluginmetadata.html)
#[repr(C)]
pub struct KPluginMetaData {
    _cspec: MaybeUninit<usize>,
}

impl Default for KPluginMetaData {
    fn default() -> Self {
        ffi::kpluginmetadata_init_default()
    }
}

impl Drop for KPluginMetaData {
    fn drop(&mut self) {
        ffi::kpluginmetadata_drop(self);
    }
}

// Safety:

// Static checks on the C++ side ensure that KPluginMetaData is trivial.
unsafe impl ExternType for KPluginMetaData {
    type Id = type_id!("KPluginMetaData");
    type Kind = cxx::kind::Trivial;
}

#[cfg(test)]
mod tests {
    use super::*;
    use cxx_qt_lib::QString;

    #[test]
    fn test() {

        let md = KPluginMetaData::default();

        assert_eq!(md.is_valid(), false);

        assert_eq!(md.is_static_plugin(), false);

        assert_eq!(md.name(), QString::default());
    }
}
