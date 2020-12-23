use bugreport::{bugreport, collectors::*, CrateInfo, Result};

struct MyCollector {}

impl Collector for MyCollector {
    fn description(&self) -> &str {
        "My collector"
    }

    fn collect(&mut self, _: &CrateInfo) -> Result<String> {
        Ok("custom info".into())
    }
}

fn main() {
    bugreport!()
        .info(SoftwareVersion::new())
        .info(MyCollector {})
        .print();
}
