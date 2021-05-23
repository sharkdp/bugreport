use std::process::Command as StdCommand;

use predicates::prelude::*;

use assert_cmd::cargo::CommandCargoExt;
use assert_cmd::Command;

pub fn assert_bin_stdout(bin: &str, regex: &str) {
    Command::from_std(StdCommand::cargo_bin(bin).unwrap())
        .assert()
        .stdout(predicates::str::is_match(regex).unwrap().normalize());
}
