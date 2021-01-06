use bugreport::{bugreport, collector::*, format::Markdown};

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
        .info(CommandOutput::new("System information", "uname", &["-a"]))
        .info(FileContent::new("Load average", "/proc/loadavg"))
        .info(CompileTimeInformation::default())
        .print::<Markdown>();
}
