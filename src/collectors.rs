use std::ffi::{OsStr, OsString};
use sys_info::{os_release, os_type};

use super::CrateInfo;
use super::Result;

pub trait Collector {
    fn description<'a>(&'a self) -> &'a str;
    fn collect(&mut self, crate_info: &CrateInfo) -> Result<String>;
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

    fn collect(&mut self, crate_info: &CrateInfo) -> Result<String> {
        Ok(format!(
            "{} {}",
            crate_info.pkg_name,
            self.version.as_deref().unwrap_or(&crate_info.pkg_version)
        ))
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

    fn collect(&mut self, _: &CrateInfo) -> Result<String> {
        let mut result = String::from("```\n");

        for arg in std::env::args_os() {
            result += &snailquote::escape(arg.to_string_lossy().as_ref());
            result += " ";
        }

        result += "\n```";
        Ok(result)
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

    fn collect(&mut self, _: &CrateInfo) -> Result<String> {
        Ok(format!("{} {}", os_type().unwrap(), os_release().unwrap()))
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

    fn collect(&mut self, _: &CrateInfo) -> Result<String> {
        let mut result = String::from("```\n");

        for var in &self.list {
            let value = std::env::var_os(&var).map(|value| value.to_string_lossy().into_owned());
            let value: Option<String> = value.map(|v| snailquote::escape(&v).into());

            result += &format!(
                "{}={}\n",
                var.to_string_lossy(),
                value.unwrap_or_else(|| "<not set>".into())
            );
        }

        result += "\n```";
        Ok(result)
    }
}
