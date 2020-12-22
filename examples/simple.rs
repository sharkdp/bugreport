use sys_info_collector::collectors::*;
use sys_info_collector::Report;

fn main() {
    Report::new("simple")
        .add(SoftwareVersion::new("1.2.3"))
        .add(OperatingSystem::new())
        .add(CommandLine::new())
        .add(EnvironmentVariables::list(&[
            "SIMPLE_CONFIG",
            "SIMPLE_THEME",
        ]))
        .print();
}
