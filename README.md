# bugreport

![Continuous integration](https://github.com/sharkdp/bugreport/workflows/Build/badge.svg)

`bugreport` is a Rust library that helps application developers to automatically collect
information about the system and the environment that users can send along with a bug
report (similar to `git bugreport` or `ffmpeg … -report`).

**Note**: This library is in an early stage and the API is very likely to change.

## Example

```rust
use bugreport::{bugreport, collectors::*};

fn main() {
    bugreport!()
        .info(OperatingSystem::default())
        .info(SoftwareVersion::default())
        .info(CommandLine::default())
        .info(EnvironmentVariables::list(&["SHELL", "PATH"]))
        .print_markdown();
}
```

## Features

- [x] Markdown export
- [ ] Ask user for consent
- [ ] Automatic anonymization of information? (e.g.: remove `/home/username` from paths)
- [ ] Open report output in editor (instead of printing to stdout)
- [ ] Auto-detect command-line option or environment variable (like `--diagnostic`)
- [ ] JSON export (?)

## Collectors

- [x] Software version
- [x] Operating system (type, name, version)
- [ ] CPU architecture
- [x] Command line (including all arguments)
- [x] Environment variables
- [x] File contents (e.g. config files)
- [x] Run command (e.g. `less --version`)
- [ ] Compile time information (architecture, OS, time, git commit)
- [ ] Current working directory
- [ ] Date and time
- [x] User defined collectors

## Use cases / prior art

- `ffmpeg`s `-report` option
  - Interesting: "Setting the environment variable FFREPORT to any value has the same effect."
  - see also: https://ffmpeg.org/bugreports.html
- `git bugreport`
  - https://git-scm.com/docs/git-bugreport
  - git version --build-options
- `grails bugreport`
  - http://docs.grails.org/3.1.1/ref/Command%20Line/bug-report.html
