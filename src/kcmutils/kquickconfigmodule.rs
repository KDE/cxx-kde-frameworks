// SPDX-FileCopyrightText: 2024 Nicolas Fella <nicolas.fell@gmx.de>
// SPDX-License-Identifier: MPL-2.0

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-kde-frameworks/kquickconfigmodule.h");
        type KQuickConfigModule;

        include!("cxx-kde-frameworks/kpluginmetadata.h");
        type KPluginMetaData = crate::kcoreaddons::KPluginMetaData;

        #[rust_name = "needs_save"]
        fn needsSave(self: &KQuickConfigModule) -> bool;

        #[rust_name = "set_needs_save"]
        fn setNeedsSave(self: Pin<&mut KQuickConfigModule>, needs: bool);

        // Q_SIGNAL void needsSaveChanged();

        #[rust_name = "auth_action_name"]
        fn authActionName(self: &KQuickConfigModule) -> QString;

        #[rust_name = "set_auth_action_name"]
        fn setAuthActionName(self: Pin<&mut KQuickConfigModule>, action: &QString);

        // Q_SIGNAL void authActionNameChanged();

        #[rust_name = "represents_defaults"]
        fn representsDefaults(self: &KQuickConfigModule) -> bool;

        #[rust_name = "set_represents_defaults"]
        fn setRepresentsDefaults(self: Pin<&mut KQuickConfigModule>, represents_defaults: bool);

        // Q_SIGNAL void representsDefaultsChanged();

        // void setButtons(const Buttons btn);

        // Buttons buttons() const;

        // Q_SIGNAL void buttonsChanged();

        // bool needsAuthorization() const;
        #[rust_name = "needs_authorization"]
        fn needsAuthorization(self: &KQuickConfigModule) -> bool;

        // QString name() const;
        fn name(self: &KQuickConfigModule) -> QString;

        // QString description() const;
        fn description(self: &KQuickConfigModule) -> QString;

        // void setDefaultsIndicatorsVisible(bool visible);
        #[rust_name = "set_defaults_indicators_visible"]
        fn setDefaultsIndicatorsVisible(self: Pin<&mut KQuickConfigModule>, visible: bool);

        // bool defaultsIndicatorsVisible() const;
        #[rust_name = "defaults_indicators_visible"]
        fn defaultsIndicatorsVisible(self: &KQuickConfigModule) -> bool;

        // Q_SIGNAL void defaultsIndicatorsVisibleChanged();

        // KPluginMetaData metaData() const;
        fn metaData(self: &KQuickConfigModule) -> KPluginMetaData;

        // Q_SIGNAL void activationRequested(const QVariantList &args);

        fn load(self: Pin<&mut KQuickConfigModule>);

        fn save(self: Pin<&mut KQuickConfigModule>);

        fn defaults(self: Pin<&mut KQuickConfigModule>);

    }
}

pub use ffi::KQuickConfigModule;
