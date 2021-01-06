// Update the code example in README.md whenver this example is changed.

use bugreport::{bugreport, collector::*, format::markdown::Markdown};

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
