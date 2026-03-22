use std::result::Result;

use bugreport::{CrateInfo, bugreport, collector::*, format::Markdown, report::ReportEntry};

struct MyCollector {}

impl Collector for MyCollector {
    fn description(&self) -> &str {
        "My collector"
    }

    fn collect(&mut self, _: &CrateInfo) -> Result<ReportEntry, CollectionError> {
        Ok(ReportEntry::Text("custom information".into()))
    }
}

fn main() {
    bugreport!()
        .info(SoftwareVersion::default())
        .info(MyCollector {})
        .print::<Markdown>();
}
