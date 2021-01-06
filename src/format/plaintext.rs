use super::Format;
use crate::report::ReportEntry;

#[derive(Default)]
pub struct Plaintext {}

impl Format for Plaintext {
    fn format_section(&mut self, title: &str) -> String {
        format!("{:-^1$}\n", title, 48)
    }

    fn format_entry(&mut self, entry: &ReportEntry) -> String {
        use ReportEntry::*;

        match entry {
            Text(content) => format!("{}\n", content),
            Code(c) => format!("{}\n", c.code),
            List(entries) => {
                let mut result = String::new();
                for entry in entries {
                    result += "- ";
                    result += &self.format_entry(entry);
                }
                result
            }
            Concat(entries) => {
                let mut result = String::new();
                for entry in entries {
                    result += &self.format_entry(entry);
                }
                result
            }
        }
    }
}
