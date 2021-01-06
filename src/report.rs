//! Defines the document structure of the report. Only needed for custom collectors.

use crate::format::Format;

#[derive(Debug)]
pub struct Code {
    pub language: Option<String>,
    pub code: String,
}

#[derive(Debug)]
pub enum ReportEntry {
    Text(String),
    Code(Code),
    List(Vec<ReportEntry>),
    Concat(Vec<ReportEntry>),
}

#[derive(Debug)]
pub(crate) struct ReportSection<'a> {
    pub(crate) title: &'a str,
    pub(crate) entry: ReportEntry,
}

impl ReportEntry {}

#[derive(Debug)]
pub(crate) struct Report<'a> {
    pub(crate) sections: Vec<ReportSection<'a>>,
}

impl<'a> Report<'a> {
    pub fn format_as(&self, format: &mut impl Format) -> String {
        let mut result = String::new();
        for section in &self.sections {
            result += &format.format_section(section.title);
            result += &format.format_entry(&section.entry);
            result += "\n";
        }

        result
    }
}
