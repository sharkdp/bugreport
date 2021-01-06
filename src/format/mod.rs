//! Different formats for printing the report.

use crate::report::ReportEntry;

#[cfg(feature = "format_markdown")]
mod markdown;
#[cfg(feature = "format_plaintext")]
mod plaintext;

#[cfg(feature = "format_markdown")]
pub use markdown::Markdown;
#[cfg(feature = "format_plaintext")]
pub use plaintext::Plaintext;

pub trait Format: Default {
    fn format_section(&mut self, title: &str) -> String;
    fn format_entry(&mut self, entry: &ReportEntry) -> String;
}
