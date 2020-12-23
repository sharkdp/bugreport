use bugreport::{bugreport, collectors::*, Collector, CrateInfo, Result};

struct MyCollector {}

impl Collector for MyCollector {
    fn description(&self) -> String {
        "My collector".into()
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
