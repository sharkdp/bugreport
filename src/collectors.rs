//! Contains all builtin information collectors and the [`Collector`] trait to implement your own.

use std::ffi::{OsStr, OsString};
use std::fs;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::process::Command;

use sys_info::{os_release, os_type};

use super::CrateInfo;
use super::Result;

use crate::helper::StringExt;
use crate::report::{Code, ReportEntry};

/// Error that appeared while collecting bug report information.
#[derive(Debug)]
pub enum CollectionError {
    CouldNotRetrieve(String),
}

impl CollectionError {
    pub(crate) fn to_entry(&self) -> ReportEntry {
        use CollectionError::*;

        match self {
            CouldNotRetrieve(reason) => ReportEntry::Text(reason.clone()),
        }
    }
}

/// Implement this trait to define customized information collectors.
pub trait Collector {
    fn description(&self) -> &str;
    fn collect(&mut self, crate_info: &CrateInfo) -> Result<ReportEntry>;
}

/// The name of your crate and the current version.
pub struct SoftwareVersion {
    version: Option<String>,
}

impl SoftwareVersion {
    pub fn custom<S: AsRef<str>>(version: S) -> Self {
        Self {
            version: Some(version.as_ref().into()),
        }
    }
}

impl Default for SoftwareVersion {
    fn default() -> Self {
        Self { version: None }
    }
}

impl Collector for SoftwareVersion {
    fn description(&self) -> &str {
        "Software version"
    }

    fn collect(&mut self, crate_info: &CrateInfo) -> Result<ReportEntry> {
        Ok(ReportEntry::Text(format!(
            "{} {}",
            crate_info.pkg_name,
            self.version.as_deref().unwrap_or(&crate_info.pkg_version)
        )))
    }
}

/// Compile-time information such as the profile (release/debug) and the target triple.
pub struct CompileTimeInformation {}

impl Default for CompileTimeInformation {
    fn default() -> Self {
        Self {}
    }
}

impl Collector for CompileTimeInformation {
    fn description(&self) -> &str {
        "Compile time information"
    }

    fn collect(&mut self, _: &CrateInfo) -> Result<ReportEntry> {
        Ok(ReportEntry::List(vec![
            ReportEntry::Text(format!("Profile: {}", env!("BUGREPORT_PROFILE"))),
            ReportEntry::Text(format!("Target triple: {}", env!("BUGREPORT_TARGET"))),
            ReportEntry::Text(format!(
                "Family: {}",
                env!("BUGREPORT_CARGO_CFG_TARGET_FAMILY")
            )),
            ReportEntry::Text(format!("OS: {}", env!("BUGREPORT_CARGO_CFG_TARGET_OS"))),
            ReportEntry::Text(format!(
                "Architecture: {}",
                env!("BUGREPORT_CARGO_CFG_TARGET_ARCH")
            )),
            ReportEntry::Text(format!(
                "Pointer width: {}",
                env!("BUGREPORT_CARGO_CFG_TARGET_POINTER_WIDTH")
            )),
            ReportEntry::Text(format!(
                "Endian: {}",
                env!("BUGREPORT_CARGO_CFG_TARGET_ENDIAN")
            )),
            ReportEntry::Text(format!(
                "CPU features: {}",
                env!("BUGREPORT_CARGO_CFG_TARGET_FEATURE")
            )),
            ReportEntry::Text(format!("Host: {}", env!("BUGREPORT_HOST"))),
        ]))
    }
}

/// The full command-line: executable name and arguments to the program.
pub struct CommandLine {}

impl Default for CommandLine {
    fn default() -> Self {
        Self {}
    }
}

impl Collector for CommandLine {
    fn description(&self) -> &str {
        "Command-line"
    }

    fn collect(&mut self, _: &CrateInfo) -> Result<ReportEntry> {
        let mut result = String::new();

        for arg in std::env::args_os() {
            result += &snailquote::escape(&arg.to_string_lossy());
            result += " ";
        }

        Ok(ReportEntry::Code(Code {
            language: Some("bash".into()),
            code: result,
        }))
    }
}

/// The operating system (type and version).
pub struct OperatingSystem {}

impl Default for OperatingSystem {
    fn default() -> Self {
        Self {}
    }
}

impl Collector for OperatingSystem {
    fn description(&self) -> &str {
        "Operating system"
    }

    fn collect(&mut self, _: &CrateInfo) -> Result<ReportEntry> {
        let os_type = os_type()
            .map_err(|_| CollectionError::CouldNotRetrieve("Operating system type".into()))?;
        let os_release = os_release();
        let os_release = os_release
            .as_ref()
            .map(|t| t.deref())
            .unwrap_or("(unknown version)");
        Ok(ReportEntry::Text(format!("{} {}", os_type, os_release)))
    }
}

/// The values of the specified environment variables (if set).
pub struct EnvironmentVariables {
    list: Vec<OsString>,
}

impl EnvironmentVariables {
    pub fn list<S: AsRef<OsStr>>(list: &[S]) -> Self {
        Self {
            list: list.iter().map(|s| s.as_ref().to_os_string()).collect(),
        }
    }
}

impl Collector for EnvironmentVariables {
    fn description(&self) -> &str {
        "Environment variables"
    }

    fn collect(&mut self, _: &CrateInfo) -> Result<ReportEntry> {
        let mut result = String::new();

        for var in &self.list {
            let value = std::env::var_os(&var).map(|value| value.to_string_lossy().into_owned());
            let value: Option<String> = value.map(|v| snailquote::escape(&v).into());

            result += &format!(
                "{}={}\n",
                var.to_string_lossy(),
                value.unwrap_or_else(|| "<not set>".into())
            );
        }
        result.pop();

        Ok(ReportEntry::Code(Code {
            language: Some("bash".into()),
            code: result,
        }))
    }
}

/// The stdout and stderr output (+ exit code) of a custom command.
pub struct CommandOutput<'a> {
    title: &'a str,
    cmd: OsString,
    cmd_args: Vec<OsString>,
}

impl<'a> CommandOutput<'a> {
    pub fn new<S, T>(title: &'a str, cmd: T, args: &[S]) -> Self
    where
        T: AsRef<OsStr>,
        S: AsRef<OsStr>,
    {
        let mut cmd_args: Vec<OsString> = Vec::new();
        for a in args {
            cmd_args.push(a.into());
        }

        CommandOutput {
            title,
            cmd: cmd.as_ref().to_owned(),
            cmd_args,
        }
    }
}

impl<'a> Collector for CommandOutput<'a> {
    fn description(&self) -> &str {
        self.title
    }

    fn collect(&mut self, _: &CrateInfo) -> Result<ReportEntry> {
        let mut result = String::new();

        result += "> ";
        result += &self.cmd.to_string_lossy();
        result += " ";
        for arg in &self.cmd_args {
            result += &snailquote::escape(&arg.to_string_lossy());
            result += " ";
        }

        result += "\n";

        let output = Command::new(&self.cmd)
            .args(&self.cmd_args)
            .output()
            .map_err(|e| {
                CollectionError::CouldNotRetrieve(format!(
                    "Could not run command '{}': {}",
                    self.cmd.to_string_lossy(),
                    e
                ))
            })?;

        let utf8_decoding_error = |_| {
            CollectionError::CouldNotRetrieve(format!(
                "Error while running command '{}': output is not valid UTF-8.",
                self.cmd.to_string_lossy()
            ))
        };

        let stdout = String::from_utf8(output.stdout).map_err(utf8_decoding_error)?;
        let stderr = String::from_utf8(output.stderr).map_err(utf8_decoding_error)?;

        result += &stdout;
        result += &stderr;

        result.trim_end_inplace();

        let mut concat = vec![];
        concat.push(ReportEntry::Code(Code {
            language: None,
            code: result,
        }));

        if !output.status.success() {
            concat.push(ReportEntry::Text(format!(
                "Command failed{}.",
                output
                    .status
                    .code()
                    .map_or("".into(), |c| format!(" with exit code {}", c))
            )));
        }

        Ok(ReportEntry::Concat(concat))
    }
}

/// The full content of a text file.
pub struct FileContent<'a> {
    title: &'a str,
    path: PathBuf,
}

impl<'a> FileContent<'a> {
    pub fn new<P: AsRef<Path>>(title: &'a str, path: P) -> Self {
        Self {
            title,
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl<'a> Collector for FileContent<'a> {
    fn description(&self) -> &str {
        self.title
    }

    fn collect(&mut self, _: &CrateInfo) -> Result<ReportEntry> {
        let mut result = fs::read_to_string(&self.path).map_err(|e| {
            CollectionError::CouldNotRetrieve(format!(
                "Could not read contents of '{}': {}.",
                self.path.to_string_lossy(),
                e
            ))
        })?;

        result.trim_end_inplace();

        Ok(ReportEntry::Code(Code {
            language: None,
            code: result,
        }))
    }
}
