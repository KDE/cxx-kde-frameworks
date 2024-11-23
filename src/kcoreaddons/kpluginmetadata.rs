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

        #[rust_name = "is_valid"]
        fn isValid(self: &KPluginMetaData) -> bool;

        #[rust_name = "is_hidden"]
        fn isHidden(self: &KPluginMetaData) -> bool;

        #[rust_name = "file_name"]
        fn fileName(self: &KPluginMetaData) -> QString;

        // QJsonObject rawData() const;

        fn name(self: &KPluginMetaData) -> QString;

        fn description(self: &KPluginMetaData) -> QString;

        // QList<KAboutPerson> authors() const;

        // QList<KAboutPerson> translators() const;

        // QList<KAboutPerson> otherContributors() const;

        fn category(self: &KPluginMetaData) -> QString;

        #[rust_name = "icon_name"]
        fn iconName(self: &KPluginMetaData) -> QString;

        fn license(self: &KPluginMetaData) -> QString;

        #[rust_name = "license_text"]
        fn licenseText(self: &KPluginMetaData) -> QString;

        #[rust_name = "copyright_text"]
        fn copyrightText(self: &KPluginMetaData) -> QString;

        #[rust_name = "plugin_id"]
        fn pluginId(self: &KPluginMetaData) -> QString;

        fn version(self: &KPluginMetaData) -> QString;

        fn website(self: &KPluginMetaData) -> QString;

        #[rust_name = "bug_report_url"]
        fn bugReportUrl(self: &KPluginMetaData) -> QString;

        #[rust_name = "mime_types"]
        fn mimeTypes(self: &KPluginMetaData) -> QStringList;

        #[rust_name = "supports_mime_type"]
        fn supportsMimeType(self: &KPluginMetaData, mime_type: &QString) -> bool;

        #[rust_name = "form_factors"]
        fn formFactors(self: &KPluginMetaData) -> QStringList;

        #[rust_name = "is_enabled_by_default"]
        fn isEnabledByDefault(self: &KPluginMetaData) -> bool;

        // template<typename T>
        // bool isEnabled(const T &config) const
        // {
        //     Q_ASSERT(config.isValid());
        //     return config.readEntry(pluginId() + QLatin1String("Enabled"), isEnabledByDefault());
        // }

        // TODO make key QStringView
        fn value(self: &KPluginMetaData, key: &QString, default_value: &QString) -> QString;

        // bool value(QStringView key, bool defaultValue) const;

        // int value(QStringView key, int defaultValue) const;

        // QStringList value(QStringView key, const QStringList &defaultValue) const;

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
