#![cfg(feature = "format_markdown")]

use std::path::PathBuf;

use pretty_assertions::assert_eq;
use tempfile::tempdir;

use bugreport::{bugreport, collector::DirectoryEntries, format::Markdown};

#[test]
fn dir_not_found() {
    let actual = bugreport!()
        .info(DirectoryEntries::new("No dir", "this-dir-does-not-exist"))
        .format::<Markdown>();

    let expected = "#### No dir

'this-dir-does-not-exist' not found

";

    assert_eq!(expected, actual);
}

#[test]
fn dir_is_empty() -> Result<(), std::io::Error> {
    let empty_dir = tempdir()?;
    let empty_dir_path = empty_dir.path();

    let actual = bugreport!()
        .info(DirectoryEntries::new("Empty dir", empty_dir_path))
        .format::<Markdown>();

    let expected = format!(
        "#### Empty dir

'{}' is empty

",
        empty_dir_path.to_string_lossy()
    );

    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn dir_exists() -> Result<(), std::io::Error> {
    let dir = tempdir()?;
    let dir_path = dir.path();

    // Put a file in the dir
    let mut some_file = PathBuf::from(dir_path);
    some_file.push("file.txt");
    std::fs::write(some_file, "This is a file")?;

    // Put a dir in the dir
    let mut some_dir = PathBuf::from(dir_path);
    some_dir.push("some_dir");
    std::fs::create_dir(some_dir)?;

    let actual = bugreport!()
        .info(DirectoryEntries::new("File and dir", dir_path))
        .format::<Markdown>();

    #[allow(unused_mut)]
    let mut expected = String::from(
        "#### File and dir

- file.txt, 14 bytes
- some_dir/

",
    );

    #[cfg(windows)]
    {
        expected = expected.replace("some_dir/", "some_dir\\");
    }

    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn new() {
    DirectoryEntries::new("a", "/a");
    // Not possible yet: DirectoryEntries::new(String::from("b"), PathBuf::from("/b"));
    DirectoryEntries::new(&String::from("c"), &PathBuf::from("/c"));
    new_with_title_from_local_variable();
}

fn new_with_title_from_local_variable() -> DirectoryEntries {
    let local_variable = String::from("pretend this is dynamically constructed");
    DirectoryEntries::new(&local_variable, "/path")
}
