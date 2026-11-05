use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocumentFormat { Odf, Ooxml, Pdf, Markdown, Html, Csv, Svg }

#[derive(Debug, Clone)]
pub struct GenOfficeDocument {
    pub path: PathBuf,
    pub format: DocumentFormat,
    pub title: String,
    pub offline_ready: bool,
}

impl GenOfficeDocument {
    pub fn new(path: PathBuf, format: DocumentFormat, title: impl Into<String>) -> Self {
        Self { path, format, title: title.into(), offline_ready: false }
    }
    pub fn mark_offline_ready(mut self) -> Self {
        self.offline_ready = true;
        self
    }
    pub fn mime_type(&self) -> &'static str {
        match self.format {
            DocumentFormat::Odf => "application/vnd.oasis.opendocument.text",
            DocumentFormat::Ooxml => "application/vnd.openxmlformats-officedocument",
            DocumentFormat::Pdf => "application/pdf",
            DocumentFormat::Markdown => "text/markdown",
            DocumentFormat::Html => "text/html",
            DocumentFormat::Csv => "text/csv",
            DocumentFormat::Svg => "image/svg+xml",
        }
    }
}

#[derive(Debug, Default)]
pub struct GenOfficeSuite {
    apps: Vec<String>,
}

impl GenOfficeSuite {
    pub fn new() -> Self { Self::default() }
    pub fn register_app(&mut self, app: impl Into<String>) { self.apps.push(app.into()); }
    pub fn has_app(&self, name: &str) -> bool { self.apps.iter().any(|a| a == name) }
    pub fn apps(&self) -> &[String] { &self.apps }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn document_mime_type_pdf() {
        let doc = GenOfficeDocument::new(PathBuf::from("report.pdf"), DocumentFormat::Pdf, "Annual Report");
        assert_eq!(doc.mime_type(), "application/pdf");
    }
    #[test]
    fn suite_register_and_lookup() {
        let mut suite = GenOfficeSuite::new();
        suite.register_app("genoffice-docs");
        assert!(suite.has_app("genoffice-docs"));
        assert!(!suite.has_app("genoffice-sheet"));
    }
}
