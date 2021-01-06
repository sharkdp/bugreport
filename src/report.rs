//! The document structure of the report. Only needed for custom collectors.

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

impl ReportEntry {
    fn to_markdown(&self) -> String {
        use ReportEntry::*;

        match self {
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
                    result += &entry.to_markdown();
                }
                result
            }
            Concat(entries) => {
                let mut result = String::new();
                for entry in entries {
                    result += &entry.to_markdown();
                }
                result
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct Report<'a> {
    pub(crate) sections: Vec<ReportSection<'a>>,
}

impl<'a> Report<'a> {
    pub fn to_markdown(&self) -> String {
        let mut result = String::new();
        for section in &self.sections {
            result += &format!("#### {}\n\n", section.title);
            result += &section.entry.to_markdown();
            result += "\n";
        }

        result
    }
}
