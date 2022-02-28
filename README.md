# bugreport

[![Continuous integration](https://github.com/sharkdp/bugreport/workflows/Build/badge.svg)](https://github.com/sharkdp/bugreport/actions) [![Crates.io](https://img.shields.io/crates/v/bugreport.svg)](https://crates.io/crates/bugreport)
[![Documentation](https://docs.rs/bugreport/badge.svg)](https://docs.rs/bugreport)

`bugreport` is a Rust library that helps application developers to automatically collect
information about the system and the environment that users can send along with a bug
report (similar to `git bugreport` or `ffmpeg … -report`).

**Note**: This library is in an early stage and the API may change in the future.

## Example

The following code
```rust
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
```
generates bug report information that [looks like this](example-report.md).


## Collectors

- [x] Crate information (name, version, git hash)
- [x] Operating system (type, name, version)
- [x] Command line (including all arguments)
- [x] Environment variables (e.g. `SHELL`, `PATH`, …)
- [x] File contents (e.g. config files)
- [x] Directory contents
- [x] Command output (e.g. `bash --version`)
- [x] Compile time information (profile, target, architecture, cpu features, etc.)
- [ ] Current working directory
- [ ] Date and time
- [x] User defined collectors

## Features

- [x] Markdown export
- [ ] Open report output in editor (instead of printing to stdout, see `git bugreport`)
- [ ] Ask user for permission to gather information?
- [ ] Automatic anonymization of information? (e.g.: remove `/home/username` from paths)
- [ ] JSON export (?)

## Use cases / prior art

- `ffmpeg`s `-report` option
  - Interesting: "Setting the environment variable FFREPORT to any value has the same effect."
  - see also: https://ffmpeg.org/bugreports.html
- `git bugreport`
  - https://git-scm.com/docs/git-bugreport
  - git version --build-options
- `grails bugreport`
  - http://docs.grails.org/3.1.1/ref/Command%20Line/bug-report.html

# Related crates

Other crates that might be useful:

- [`human-panic`](https://crates.io/crates/human-panic) - Make panic messages nice for humans to read.
