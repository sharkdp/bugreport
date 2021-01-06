use bugreport::{bugreport, collector::*};

fn main() {
    bugreport!()
        .info(SoftwareVersion::default())
        .info(OperatingSystem::default())
        .info(CommandLine::default())
        .info(EnvironmentVariables::list(&["SHELL", "EDITOR"]))
        .info(CommandOutput::new("Python", "python", &["--version"]))
        .info(CompileTimeInformation::default())
        .print_markdown();
}
