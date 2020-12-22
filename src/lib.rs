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
    fn collect(&mut self, report_info: &ReportInfo) -> Result<String>;
}

pub struct ReportInfo<'a> {
    app_name: &'a str,
}

pub struct Report<'a> {
    info: ReportInfo<'a>,
    collectors: Vec<Box<dyn Collector>>,
}

impl<'a> Report<'a> {
    pub fn new(app_name: &'a str) -> Self {
        Report {
            info: ReportInfo { app_name },
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

pub mod collectors {

    use std::ffi::{OsStr, OsString};
    use sys_info::{os_release, os_type};

    use super::Collector;
    use super::ReportInfo;
    use super::Result;

    pub struct SoftwareVersion {
        version: String,
    }

    impl SoftwareVersion {
        pub fn new<S: AsRef<str>>(version: S) -> Self {
            Self {
                version: version.as_ref().into(),
            }
        }
    }

    impl Collector for SoftwareVersion {
        fn description(&self) -> String {
            "Software version".into()
        }

        fn collect(&mut self, report_info: &ReportInfo) -> Result<String> {
            Ok(format!("{} {}", report_info.app_name, self.version.clone()))
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
            "Command-line arguments".into()
        }

        fn collect(&mut self, _: &ReportInfo) -> Result<String> {
            let mut result = String::from("```\n");

            for arg in std::env::args_os() {
                result += arg.to_string_lossy().as_ref();
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

        fn collect(&mut self, _: &ReportInfo) -> Result<String> {
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

        fn collect(&mut self, _: &ReportInfo) -> Result<String> {
            let mut result = String::from("```\n");

            for var in &self.list {
                let value =
                    std::env::var_os(&var).map(|value| format!("'{}'", value.to_string_lossy()));

                result += &format!(
                    "{} = {}\n",
                    var.to_string_lossy(),
                    value.unwrap_or_else(|| "<not set>".into())
                );
            }

            result += "\n```";
            Ok(result)
        }
    }
}
