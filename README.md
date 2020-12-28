# bugreport


## Features and design goals

- Markdown export
- JSON export?
- Ask user for consent
- Automatic anonymization of information?
- Custom collectors
- Minimal dependencies
- Open output in editor
- Auto-detect command-line option or env variable

## Collectors

- Software version
- Operating system (type, name, version)
- CPU architecture
- Command line (including all arguments)
- Environment variables
- File contents (e.g. config files)
- Run command (e.g. `less --version`)
- Compile time information (architecture, OS, time, git commit)
- Current working directory?
- Date and time
- User defined collectors

## Use cases / prior art

- `ffmpeg`s `-report` option
  - Interesting: "Setting the environment variable FFREPORT to any value has the same effect."
  - see also: https://ffmpeg.org/bugreports.html
- `git bugreport`
  - https://git-scm.com/docs/git-bugreport
  - git version --build-options
- `grails bugreport`
  - http://docs.grails.org/3.1.1/ref/Command%20Line/bug-report.html
