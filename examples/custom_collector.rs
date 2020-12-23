use bugreport::{bugreport, collectors::*, report::ReportEntry, CrateInfo, Result};

struct MyCollector {}

impl Collector for MyCollector {
    fn description(&self) -> &str {
        "My collector"
    }

    fn collect(&mut self, _: &CrateInfo) -> Result<ReportEntry> {
        Ok(ReportEntry::Text("custom info".into()))
    }
}

fn main() {
    bugreport!()
        .info(SoftwareVersion::new())
        .info(MyCollector {})
        .print_markdown();
}
