use bugreport::{bugreport, collectors::*};

fn main() {
    bugreport!()
        .info(SoftwareVersion::new())
        .info(OperatingSystem::new())
        .info(CommandLine::new())
        .info(EnvironmentVariables::list(&[
            "SHELL",
            "PATH",
            "SIMPLE_VAR_1",
            "SIMPLE_VAR_2",
        ]))
        .info(CommandOutput::new(
            "System information",
            std::ffi::OsStr::new("uname"),
            &["-a"],
        ))
        .print_markdown();
}
