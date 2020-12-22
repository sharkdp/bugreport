use sys_info_collector::collectors::*;
use sys_info_collector::Report;

fn main() {
    Report::new()
        .info(SoftwareVersion::new())
        .info(OperatingSystem::new())
        .info(CommandLine::new())
        .info(EnvironmentVariables::list(&[
            "SIMPLE_CONFIG",
            "SIMPLE_THEME",
        ]))
        .print();
}
