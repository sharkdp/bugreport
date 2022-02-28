use std::fs::{self, DirEntry};
use std::io::ErrorKind;
use std::path::PathBuf;

use crate::{report::ReportEntry, Collector, CrateInfo, Result};

use super::CollectionError;

/// List information about entries in a directory.
///
/// Limitations:
/// * Is not recursive
/// * Does not handle symbolic links
/// * Only sizes of files are printed and not e.g. time of last modification
///
/// # Example
///
/// ```md
/// #### File and dir
///
/// - file.txt, 14 bytes
/// - some_dir
///
/// ```
pub struct DirectoryEntries {
    title: String,
    path: PathBuf,
}

impl DirectoryEntries {
    pub fn new(title: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self {
            title: title.into(),
            path: path.into(),
        }
    }
}

impl Collector for DirectoryEntries {
    fn description(&self) -> &str {
        &self.title
    }

    fn collect(&mut self, _: &CrateInfo) -> Result<ReportEntry> {
        let path_str = &self.path.to_string_lossy();

        let mut entries = fs::read_dir(&self.path)
            .map_err(|e| read_dir_error_to_report_entry(e, path_str))?
            .map(|e| match e {
                Ok(dir_entry) => dir_entry_to_report_entry(dir_entry),
                Err(e) => format!("Error: {}", e),
            })
            .collect::<Vec<_>>();

        // For stable ordering
        entries.sort();

        if entries.is_empty() {
            Ok(ReportEntry::Text(format!("'{}' is empty", path_str)))
        } else {
            Ok(ReportEntry::List(
                entries.into_iter().map(ReportEntry::Text).collect(),
            ))
        }
    }
}

fn read_dir_error_to_report_entry(error: std::io::Error, path_str: &str) -> CollectionError {
    CollectionError::CouldNotRetrieve(if error.kind() == ErrorKind::NotFound {
        format!("'{}' not found", path_str)
    } else {
        format!("'{}' not read: {}", path_str, error)
    })
}

fn dir_entry_to_report_entry(dir_entry: DirEntry) -> String {
    let mut text = String::new();

    text.push_str(&dir_entry.file_name().to_string_lossy());

    if let Ok(metadata) = dir_entry.metadata() {
        if metadata.is_file() {
            text.push_str(&format!(", {} bytes", metadata.len()));
        }
    }

    text
}
