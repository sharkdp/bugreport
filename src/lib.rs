// Ideas:
// - ask user for consent
// - configure what app wants to collect
// - different serialization formats: markdown, json

// Info:
//
// - Architecture
// - SW version
// - File contents
//
// - env variables
// - command line arguments
//

use sys_info::{os_release, os_type};

#[derive(Debug)]
pub struct Error;

type Result<T> = std::result::Result<T, Error>;

pub trait Collector {
    fn description(&self) -> String;
    fn collect(&mut self) -> Result<String>;
}

pub struct Report {
    collectors: Vec<Box<dyn Collector>>,
}

impl Report {
    pub fn new() -> Self {
        Report { collectors: vec![] }
    }

    pub fn add<C: Collector + 'static>(mut self, collector: C) -> Self {
        self.collectors.push(Box::new(collector));
        self
    }

    pub fn generate(&mut self) -> String {
        let mut report = String::new();

        for collector in &mut self.collectors {
            report += "## ";
            report += &collector.description();
            report += "\n\n";
            report += &collector.collect().unwrap();
            report += "\n\n";
        }
        report
    }
}

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

    fn collect(&mut self) -> Result<String> {
        Ok(self.version.clone())
    }
}

pub struct CommandLineArguments {}

impl CommandLineArguments {
    pub fn new() -> Self {
        Self {}
    }
}

impl Collector for CommandLineArguments {
    fn description(&self) -> String {
        "Command-line arguments".into()
    }

    fn collect(&mut self) -> Result<String> {
        let mut result = String::from("[");

        for arg in std::env::args() {
            result += &format!("'{}', ", arg);
        }

        result += "]";
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

    fn collect(&mut self) -> Result<String> {
        Ok(format!("{} {}", os_type().unwrap(), os_release().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let report = Report::new()
            .add(OperatingSystem::new())
            .add(SoftwareVersion::new("1.3.4"))
            .add(CommandLineArguments::new())
            .generate();

        println!("{}", "Report:");
        println!("{}", report);
        println!("{}", "===");
    }
}
