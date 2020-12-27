use bugreport::{bugreport, collectors::*};

fn main() {
    bugreport!()
        .info(SoftwareVersion::new())
        .info(OperatingSystem::new())
        .info(CommandLine::new())
        .info(EnvironmentVariables::list(&[
            "SIMPLE_CONFIG",
            "SIMPLE_THEME",
        ]))
        .info(CommandOutput::new(
            "less version",
            std::ffi::OsStr::new("less"),
            &["--version"],
        ))
        .print_markdown();
}
