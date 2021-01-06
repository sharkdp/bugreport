//! Different formats for printing the report.

use crate::report::ReportEntry;

#[cfg(feature = "format_markdown")]
pub mod markdown;
#[cfg(feature = "format_plaintext")]
pub mod plaintext;

pub trait Format: Default {
    fn format_section(&mut self, title: &str) -> String;
    fn format_entry(&mut self, entry: &ReportEntry) -> String;
}
