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

        /// Returns true when the module has something changed and needs save.
        ///
        /// [C++ API documentation](https://api.kde.org/kabstractconfigmodule.html#needsSave)
        #[rust_name = "needs_save"]
        fn needsSave(self: &KQuickConfigModule) -> bool;

        /// Set this property to true when the user changes something in the module, signaling that it needs a save (such as user pressing Ok or Apply).
        ///
        /// [C++ API documentation](https://api.kde.org/kabstractconfigmodule.html#setNeedsSave)
        #[rust_name = "set_needs_save"]
        fn setNeedsSave(self: Pin<&mut KQuickConfigModule>, needs: bool);

        // Q_SIGNAL void needsSaveChanged();

        /// Returns the action previously set with setAuthActionName() and that is authorized to execute the save() method.
        ///
        /// [C++ API documentation](https://api.kde.org/kabstractconfigmodule.html#authActionName)
        #[rust_name = "auth_action_name"]
        fn authActionName(self: &KQuickConfigModule) -> QString;

        /// Set if the module's save() method requires authorization to be executed.
        ///
        /// [C++ API documentation](https://api.kde.org/kabstractconfigmodule.html#setAuthActionName)
        #[rust_name = "set_auth_action_name"]
        fn setAuthActionName(self: Pin<&mut KQuickConfigModule>, action: &QString);

        // Q_SIGNAL void authActionNameChanged();

        /// Returns true when the module state represents the default settings.
        ///
        /// [C++ API documentation](https://api.kde.org/kabstractconfigmodule.html#representsDefaults)
        #[rust_name = "represents_defaults"]
        fn representsDefaults(self: &KQuickConfigModule) -> bool;

        /// Set this property to true when the user sets the state of the module to the defaults settings (e.g. clicking Defaults would do nothing).
        ///
        /// [C++ API documentation](https://api.kde.org/kabstractconfigmodule.setRepresentsDefaults)
        #[rust_name = "set_represents_defaults"]
        fn setRepresentsDefaults(self: Pin<&mut KQuickConfigModule>, represents_defaults: bool);

        // Q_SIGNAL void representsDefaultsChanged();

        // void setButtons(const Buttons btn);

        // Buttons buttons() const;

        // Q_SIGNAL void buttonsChanged();

        /// Returns true if the authActionName is not empty.
        ///
        /// [C++ API documentation](https://api.kde.org/kabstractconfigmodule.html#needsAuthorization)
        #[rust_name = "needs_authorization"]
        fn needsAuthorization(self: &KQuickConfigModule) -> bool;

        /// Returns the name of the config module.
        ///
        /// [C++ API documentation](https://api.kde.org/kabstractconfigmodule.html#name)
        fn name(self: &KQuickConfigModule) -> QString;

        /// Returns the description of the config module.
        ///
        /// [C++ API documentation](https://api.kde.org/kabstractconfigmodule.html#description)
        fn description(self: &KQuickConfigModule) -> QString;

        /// Changes the defaultness indicator visibility.
        ///
        /// [C++ API documentation](https://api.kde.org/kabstractconfigmodule.html#setDefaultsIndicatorsVisible)
        #[rust_name = "set_defaults_indicators_visible"]
        fn setDefaultsIndicatorsVisible(self: Pin<&mut KQuickConfigModule>, visible: bool);

        /// Returns the defaultness indicator visibility.
        ///
        /// [C++ API documentation](https://api.kde.org/kabstractconfigmodule.html#defaultsIndicatorsVisible)
        #[rust_name = "defaults_indicators_visible"]
        fn defaultsIndicatorsVisible(self: &KQuickConfigModule) -> bool;

        // Q_SIGNAL void defaultsIndicatorsVisibleChanged();

        /// Returns the metaData that was used when instantiating the plugin.
        ///
        /// [C++ API documentation](https://api.kde.org/kabstractconfigmodule.html#metaData)
        fn metaData(self: &KQuickConfigModule) -> KPluginMetaData;

        // Q_SIGNAL void activationRequested(const QVariantList &args);

        /// Loads the configuration data into the module.
        ///
        /// [C++ API documentation](https://api.kde.org/kabstractconfigmodule.html#load)
        fn load(self: Pin<&mut KQuickConfigModule>);

        /// The save method stores the config information as shown in the user interface in the config files.
        ///
        /// [C++ API documentation](https://api.kde.org/kabstractconfigmodule.html#save)
        fn save(self: Pin<&mut KQuickConfigModule>);

        /// Sets the configuration to default values.
        ///
        /// [C++ API documentation](https://api.kde.org/kabstractconfigmodule.html#defaults)
        fn defaults(self: Pin<&mut KQuickConfigModule>);

    }
}

pub use ffi::KQuickConfigModule;
