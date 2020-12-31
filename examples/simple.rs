use bugreport::{bugreport, collectors::*};

fn main() {
    bugreport!()
        .info(SoftwareVersion::default())
        .info(OperatingSystem::default())
        .info(CommandLine::default())
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
