use std::result;

pub mod collectors;

#[derive(Debug)]
pub enum CollectionError {}

pub type Result<T> = result::Result<T, CollectionError>;

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
