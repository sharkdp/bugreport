use sys_info_collector::collectors::*;
use sys_info_collector::Report;

fn main() {
    Report::new("simple")
        .info(SoftwareVersion::new("1.2.3"))
        .info(OperatingSystem::new())
        .info(CommandLine::new())
        .info(EnvironmentVariables::list(&[
            "SIMPLE_CONFIG",
            "SIMPLE_THEME",
        ]))
        .print();
}
