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
    git_hash: Option<&'a str>,
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
                git_hash: None,
            },
            collectors: vec![],
        }
    }

    #[doc(hidden)]
    pub fn set_git_hash(&mut self, git_hash: Option<&'a str>) {
        self.info.git_hash = git_hash;
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

/// Re-export so dependent project does not have to manually depend on git-version crate
#[cfg(feature = "git_hash")]
pub use git_version::git_version;

#[cfg(feature = "git_hash")]
#[doc(hidden)]
#[macro_export]
macro_rules! bugreport_set_git_hash {
    ($br:ident) => {{
        let hash = bugreport::git_version!(fallback = "");
        if !hash.is_empty() {
            $br.set_git_hash(Some(hash));
        }
    }};
}

#[cfg(not(feature = "git_hash"))]
#[doc(hidden)]
#[macro_export]
macro_rules! bugreport_set_git_hash {
    ($br:ident) => {};
}

/// Generate a new [`BugReport`] object.
#[macro_export]
macro_rules! bugreport {
    () => {{
        let mut br = bugreport::BugReport::from_name_and_version(
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
        );
        bugreport::bugreport_set_git_hash!(br);
        br
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "format_markdown")]
    fn basic() {
        use super::BugReport;
        use crate::collector::*;
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
