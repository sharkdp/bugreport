// Ideas:
// - configure what app wants to collect
// - different serialization formats: markdown, json
// - ask user for consent
// - anonymization: e.g. anonymize user name

// Info:
//
// - Architecture
// - SW version
// - File contents
// - Compile time information:
//    - std::env::consts::ARCH, std::env::consts::OS
//    - Full Target triple?
//    - Time
//    - Git commit
// - Current working directory
// - std::env::current_exe
// - User-defined reporter
// - env variables
// - command line arguments
//

#[derive(Debug)]
pub enum CollectionError {}

type Result<T> = std::result::Result<T, CollectionError>;

pub trait Collector {
    fn description(&self) -> String;
    fn collect(&mut self, crate_info: &CrateInfo) -> Result<String>;
}

pub struct CrateInfo<'a> {
    pkg_name: &'a str,
    pkg_version: &'a str,
}

pub struct BugReport<'a> {
    info: CrateInfo<'a>,
    collectors: Vec<Box<dyn Collector>>,
}

impl<'a> BugReport<'a> {
    pub fn new(pkg_name: &'a str, pkg_version: &'a str) -> Self {
        BugReport {
            info: CrateInfo {
                pkg_name,
                pkg_version,
            },
            collectors: vec![],
        }
    }

    pub fn info<C: Collector + 'static>(mut self, collector: C) -> Self {
        self.collectors.push(Box::new(collector));
        self
    }

    pub fn generate(&mut self) -> String {
        let mut report = String::new();

        for collector in &mut self.collectors {
            report += "## ";
            report += &collector.description();
            report += "\n\n";
            report += &collector.collect(&self.info).unwrap();
            report += "\n\n";
        }
        report
    }

    pub fn print(&mut self) {
        print!("{}", self.generate());
    }
}

#[macro_export]
macro_rules! bugreport {
    () => {
        bugreport::BugReport::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
    };
}

pub mod collectors {

    use std::ffi::{OsStr, OsString};
    use sys_info::{os_release, os_type};

    use super::Collector;
    use super::CrateInfo;
    use super::Result;

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
        fn description(&self) -> String {
            "Software version".into()
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
        fn description(&self) -> String {
            "Command-line".into()
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
        fn description(&self) -> String {
            "Operating system".into()
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
        fn description(&self) -> String {
            "Environment variables".into()
        }

        fn collect(&mut self, _: &CrateInfo) -> Result<String> {
            let mut result = String::from("```\n");

            for var in &self.list {
                let value =
                    std::env::var_os(&var).map(|value| value.to_string_lossy().into_owned());
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
}
