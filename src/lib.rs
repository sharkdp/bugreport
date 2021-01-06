//! `bugreport` is a library that helps application developers to automatically collect
//! information about the system and the environment that users can send along with a bug
//! report (similar to `git bugreport` or `ffmpeg … -report`).
//!
//! Usage example:
//! ```
//! use bugreport::{bugreport, collector::*, format::Markdown};
//!
//! bugreport!()
//!     .info(SoftwareVersion::default())
//!     .info(OperatingSystem::default())
//!     .info(CommandLine::default())
//!     .info(EnvironmentVariables::list(&["SHELL", "EDITOR"]))
//!     .info(CommandOutput::new("Python version", "python", &["--version"]))
//!     .info(CompileTimeInformation::default())
//!     .print::<Markdown>();
//! ```

use std::result;

pub mod collector;
pub mod format;
mod helper;
pub mod report;

use collector::{CollectionError, Collector};
use format::Format;
use report::{Report, ReportSection};

pub(crate) type Result<T> = result::Result<T, CollectionError>;

#[doc(hidden)]
pub struct CrateInfo<'a> {
    pkg_name: &'a str,
    pkg_version: &'a str,
}

/// The main struct for collecting bug report information.
///
/// Use the [`bugreport`] macro to create one.
pub struct BugReport<'a> {
    info: CrateInfo<'a>,
    collectors: Vec<Box<dyn Collector>>,
}

impl<'a> BugReport<'a> {
    #[doc(hidden)]
    pub fn from_name_and_version(pkg_name: &'a str, pkg_version: &'a str) -> Self {
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

    /// Assemble the bug report information using the given format.
    pub fn format<F: Format>(&mut self) -> String {
        let mut format = F::default();
        self.generate().format_as(&mut format)
    }

    /// Print the bug report information using the given format.
    pub fn print<F: Format>(&mut self) {
        println!("{}", self.format::<F>());
    }
}

/// Generate a new [`BugReport`] object.
#[macro_export]
macro_rules! bugreport {
    () => {
        bugreport::BugReport::from_name_and_version(
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collector::*;

    #[test]
    #[cfg(feature = "format_markdown")]
    fn basic() {
        use crate::format::Markdown;

        std::env::set_var("BUGREPORT_TEST", "42");

        let report = BugReport::from_name_and_version("dummy", "0.1")
            .info(EnvironmentVariables::list(&["BUGREPORT_TEST"]))
            .format::<Markdown>();

        assert_eq!(
            report,
            "#### Environment variables\n\
             \n\
             ```bash\n\
             BUGREPORT_TEST=42\n\
             ```\n\n"
        );
    }
}
