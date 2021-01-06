//! Different formats for printing the report.

use crate::report::ReportEntry;

pub mod markdown;
pub mod plaintext;

pub trait Format: Default {
    fn format_section(&mut self, title: &str) -> String;
    fn format_entry(&mut self, entry: &ReportEntry) -> String;
}
