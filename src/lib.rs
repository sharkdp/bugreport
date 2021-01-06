//! [`bugreport`] is a Rust library that helps application developers to automatically collect
//! information about the system and the environment that users can send along with a bug
//! report (similar to `git bugreport` or `ffmpeg … -report`).
//!
//! Usage example:
//! ```
//! use bugreport::{bugreport, collectors::*};
//!
//! bugreport!()
//!     .info(SoftwareVersion::default())
//!     .info(OperatingSystem::default())
//!     .info(CommandLine::default())
//!     .info(EnvironmentVariables::list(&["SHELL", "EDITOR"]))
//!     .info(CommandOutput::new("Python version", "python", &["--version"]))
//!     .info(CompileTimeInformation::default())
//!     .print_markdown();
//! ```

use std::result;

pub mod collectors;
mod helper;
pub mod report;

use collectors::{CollectionError, Collector};
use report::{Report, ReportSection};

pub(crate) type Result<T> = result::Result<T, CollectionError>;

#[doc(hidden)]
pub struct CrateInfo<'a> {
    pkg_name: &'a str,
    pkg_version: &'a str,
}

/// A builder for the bug report. Use the [`bugreport`] macro to create one.
pub struct BugReport<'a> {
    info: CrateInfo<'a>,
    collectors: Vec<Box<dyn Collector>>,
}

impl<'a> BugReport<'a> {
    #[doc(hidden)]
    pub fn new(pkg_name: &'a str, pkg_version: &'a str) -> Self {
        BugReport {
            info: CrateInfo {
                pkg_name,
                pkg_version,
            },
            collectors: vec![],
        }
    }

    /// Add a [`Collector`] to the bug report.
    pub fn info<C: Collector + 'static>(mut self, collector: C) -> Self {
        self.collectors.push(Box::new(collector));
        self
    }

    fn generate(&mut self) -> Report {
        let mut sections = vec![];

        for collector in &mut self.collectors {
            let entry = collector
                .collect(&self.info)
                .unwrap_or_else(|e| e.to_entry());
            sections.push(ReportSection {
                title: collector.description(),
                entry,
            });
        }

        Report { sections }
    }

    /// Generate the bug report information as Markdown.
    pub fn format_markdown(&mut self) -> String {
        self.generate().to_markdown()
    }

    /// Print the bug report information as Markdown.
    pub fn print_markdown(&mut self) {
        println!("{}", self.format_markdown());
    }
}

/// Generate a new [`BugReport`] object.
///
/// Example usage:
/// ```
/// use bugreport::{bugreport, collectors::*};
///
/// bugreport!()
///         .info(SoftwareVersion::default())
///         .print_markdown();
/// ```
///
#[macro_export]
macro_rules! bugreport {
    () => {
        bugreport::BugReport::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
    };
}
