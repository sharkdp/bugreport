use std::result;

pub mod collectors;
mod helper;
pub mod report;

use collectors::{CollectionError, Collector};
use report::{Report, ReportSection};

pub type Result<T> = result::Result<T, CollectionError>;

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

    pub fn generate(&mut self) -> Report {
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

    pub fn print_markdown(&mut self) {
        let report = self.generate();
        print!("{}", report.to_markdown());
    }
}

#[macro_export]
macro_rules! bugreport {
    () => {
        bugreport::BugReport::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
    };
}
