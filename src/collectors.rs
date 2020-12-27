use std::ffi::{OsStr, OsString};
use std::process::Command;

use sys_info::{os_release, os_type};

use super::CrateInfo;
use super::Result;

use crate::report::{Code, ReportEntry};

#[derive(Debug)]
pub enum CollectionError {
    CouldNotRetrieve(String),
}

impl CollectionError {
    pub(crate) fn to_entry(&self) -> ReportEntry {
        use CollectionError::*;

        match self {
            CouldNotRetrieve(what) => ReportEntry::Text(format!("Could not retrieve {}", what)),
        }
    }
}

pub trait Collector {
    fn description(&self) -> &str;
    fn collect(&mut self, crate_info: &CrateInfo) -> Result<ReportEntry>;
}

pub struct SoftwareVersion {
    version: Option<String>,
}

impl SoftwareVersion {
    pub fn new() -> Self {
        Self { version: None }
    }

    pub fn custom<S: AsRef<str>>(version: S) -> Self {
        Self {
            version: Some(version.as_ref().into()),
        }
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

pub struct CommandLine {}

impl CommandLine {
    pub fn new() -> Self {
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

pub struct OperatingSystem {}

impl OperatingSystem {
    pub fn new() -> Self {
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
        let os_release = os_release().unwrap_or("(unknown version)".into());
        Ok(ReportEntry::Text(format!("{} {}", os_type, os_release)))
    }
}

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

pub struct CommandOutput<'a> {
    title: &'a str,
    cmd: &'a OsStr,
    cmd_args: Vec<&'a OsStr>,
}

impl<'a> CommandOutput<'a> {
    pub fn new<S>(title: &'a str, cmd: &'a OsStr, args: &'a [S]) -> Self
    where
        S: AsRef<OsStr> + 'a,
    {
        let mut cmd_args = Vec::new();
        for a in args {
            cmd_args.push(a.as_ref());
        }

        CommandOutput {
            title,
            cmd: cmd.as_ref(),
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

        let output = Command::new(self.cmd)
            .args(&self.cmd_args)
            .output()
            .map_err(|_| CollectionError::CouldNotRetrieve("TODO".into()))?;

        // TODO: stderr, exit code
        let stdout = String::from_utf8(output.stdout)
            .map_err(|_| CollectionError::CouldNotRetrieve("TODO".into()))?;

        result += &stdout;

        // trim in place
        result.truncate(result.trim_end().len());

        Ok(ReportEntry::Code(Code {
            language: None,
            code: result,
        }))
    }
}
