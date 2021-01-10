// Update the code example in README.md whenever this example is changed.

use bugreport::{bugreport, collector::*, format::Markdown};

fn main() {
    bugreport!()
        .info(SoftwareVersion::default())
        .info(OperatingSystem::default())
        .info(CommandLine::default())
        .info(EnvironmentVariables::list(&["SHELL", "EDITOR"]))
        .info(CommandOutput::new("Python version", "python", &["-V"]))
        .info(CompileTimeInformation::default())
        .print::<Markdown>();
}
