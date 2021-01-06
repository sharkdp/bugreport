use super::Format;
use crate::report::ReportEntry;

#[derive(Default)]
pub struct Markdown {}

impl Format for Markdown {
    fn format_section(&mut self, title: &str) -> String {
        format!("#### {}\n\n", title)
    }

    fn format_entry(&mut self, entry: &ReportEntry) -> String {
        use ReportEntry::*;

        match entry {
            Text(content) => format!("{}\n", content),
            Code(c) => format!(
                "```{}\n{}\n```\n",
                c.language.as_deref().unwrap_or(""),
                c.code
            ),
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
