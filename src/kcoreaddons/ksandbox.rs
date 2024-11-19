#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        type QProcessEnvironment;

        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = cxx_qt_lib::QStringList;

        fn insert(self: Pin<&mut QProcessEnvironment>, name: &QString, value: &QString);
        fn remove(self: Pin<&mut QProcessEnvironment>, name: &QString);
    }

    #[namespace = "rust::kf6"]
    unsafe extern "C++" {
        include!("cxx-kde-frameworks/ksandbox.h");

        #[rust_name = "make_host_context"]
        fn makeHostContext(
            working_directory: &QString,
            program: &QString,
            arguments: &QStringList,
            env: &QProcessEnvironment,
        ) -> UniquePtr<ProcessContext>;

        #[rust_name = "system_environment"]
        fn systemEnvironment() -> UniquePtr<QProcessEnvironment>;

        #[rust_name = "get_program"]
        fn contextGetProgram(ctx: &ProcessContext) -> QString;

        #[rust_name = "get_arguments"]
        fn contextGetArgs(ctx: &ProcessContext) -> QStringList;
    }

    #[namespace = "KSandbox"]
    unsafe extern "C++" {
        type ProcessContext;

        #[rust_name = "is_inside"]
        fn isInside() -> bool;

        #[rust_name = "is_flatpak"]
        fn isFlatpak() -> bool;

        #[rust_name = "is_snap"]
        fn isSnap() -> bool;
    }
}

pub use ffi::{is_flatpak, is_inside, is_snap};

use cxx::UniquePtr;
use cxx_qt_lib::{QList, QString, QStringList};
use ffi::{system_environment, QProcessEnvironment};
use std::{collections::HashMap, path::PathBuf, process::{Command, Stdio}};

pub struct HostCommand {
    program: QString,
    arguments: QList<QString>,
    workdir: Option<String>,
    stdin: Option<Stdio>,
    stdout: Option<Stdio>,
    stderr: Option<Stdio>,
    env: HashMap<String, Option<String>>,
}

impl HostCommand {
    pub fn new(program: &str) -> HostCommand {
        HostCommand {
            program: QString::from(program),
            arguments: QList::default(),
            workdir: None,
            stdin: None,
            stdout: None,
            stderr: None,
            env: HashMap::new(),
        }
    }

    pub fn arg(mut self, arg: &str) -> HostCommand {
        self.arguments.append(QString::from(arg));
        self
    }

    pub fn args(mut self, args: &[&str]) -> HostCommand {
        self.arguments
            .reserve(self.arguments.len() + args.len() as isize);
        for arg in args {
            self.arguments.append(QString::from(*arg));
        }
        self
    }

    pub fn current_dir(mut self, dir: &str) -> HostCommand {
        self.workdir = Some(dir.to_string());
        self
    }

    pub fn stdin<T: Into<Stdio>>(mut self, cfg: T) -> HostCommand {
        self.stdin = Some(cfg.into());
        self
    }

    pub fn stdout<T: Into<Stdio>>(mut self, cfg: T) -> HostCommand {
        self.stdout = Some(cfg.into());
        self
    }

    pub fn stderr<T: Into<Stdio>>(mut self, cfg: T) -> HostCommand {
        self.stderr = Some(cfg.into());
        self
    }

    pub fn env(mut self, key: &str, value: &str) -> HostCommand {
        self.env.insert(key.to_string(), Some(value.to_string()));
        self
    }

    pub fn env_remove(mut self, key: &str) -> HostCommand {
        self.env.insert(key.to_string(), None);
        self
    }

    fn qt_env(&self) -> UniquePtr<QProcessEnvironment> {
        let mut env = system_environment();
        for (key, value) in &self.env {
            match value {
                Some(value) => {
                    let env_pin = env.as_mut().expect("C++ code should always return a valid pointer");
                    env_pin.insert(&QString::from(key), &QString::from(value));
                }
                None => {
                    let env_pin = env.as_mut().expect("C++ code should always return a valid pointer");
                    env_pin.remove(&QString::from(key));
                }
            }
        }
        env
    }

    ///
    /// Create a [std::process::Command] that runs the command on the host
    ///
    pub fn command(&self) -> Command {
        // Call KCoreAddons API
        let ctx = ffi::make_host_context(
            &self.workdir.as_ref().map(QString::from).unwrap_or_default(),
            &self.program,
            &QStringList::from(&self.arguments),
            &self.qt_env(),
        );

        // Convert result back to rust types
        let program = ffi::get_program(
            ctx.as_ref()
                .expect("C++ code should always return a valid pointer"),
        )
        .to_string();
        let arguments = QList::<QString>::from(&ffi::get_arguments(
            ctx.as_ref()
                .expect("C++ code should always return a valid pointer"),
        ));

        let mut args = Vec::with_capacity(arguments.len() as usize);
        for arg in arguments.iter() {
            args.push(arg.to_string())
        }

        let mut host_command = Command::new(program);
        host_command.args(&args);

        for (key, value) in &self.env {
            match value {
                Some(value) => {
                    host_command.env(key, value);
                }
                None => {
                    host_command.env_remove(key);
                }
            }
        }

        if let Some(workdir) = &self.workdir {
            host_command.current_dir(&PathBuf::from(&workdir));
        }

        host_command
    }
}

#[test]
fn test() {
    let mut command = HostCommand::new("kdialog").arg("--error").arg("It works").env("QT_QPA_PLATFORM", "wayland").env("LANG", "C.UTF-8").current_dir("/tmp/").command();
    eprintln!("{command:?}");
    command.spawn();
}
