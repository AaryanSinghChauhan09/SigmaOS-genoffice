#![forbid(unsafe_code)]
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocumentFormat {
    Odf,
    Ooxml,
    Pdf,
    Markdown,
    Html,
    Csv,
    Svg,
}

impl DocumentFormat {
    /// Detect the format from a file extension (lowercased, no dot).
    /// Returns `None` for unknown or missing extensions.
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_ascii_lowercase().as_str() {
            "odt" | "ods" | "odp" | "odg" | "odf" => Some(Self::Odf),
            "docx" | "xlsx" | "pptx" | "docm" | "xlsm" | "pptm" => Some(Self::Ooxml),
            "pdf" => Some(Self::Pdf),
            "md" | "markdown" | "mdown" => Some(Self::Markdown),
            "html" | "htm" | "xhtml" => Some(Self::Html),
            "csv" | "tsv" => Some(Self::Csv),
            "svg" => Some(Self::Svg),
            _ => None,
        }
    }

    /// Detect the format from a path's extension.
    pub fn from_path(path: &std::path::Path) -> Option<Self> {
        Self::from_extension(path.extension()?.to_str()?)
    }
}

#[derive(Debug, Clone)]
pub struct GenOfficeDocument {
    pub path: PathBuf,
    pub format: DocumentFormat,
    pub title: String,
    pub offline_ready: bool,
}

impl GenOfficeDocument {
    pub fn new(path: PathBuf, format: DocumentFormat, title: impl Into<String>) -> Self {
        Self {
            path,
            format,
            title: title.into(),
            offline_ready: false,
        }
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
    pub fn new() -> Self {
        Self::default()
    }
    pub fn register_app(&mut self, app: impl Into<String>) {
        self.apps.push(app.into());
    }
    pub fn has_app(&self, name: &str) -> bool {
        self.apps.iter().any(|a| a == name)
    }
    pub fn apps(&self) -> &[String] {
        &self.apps
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn document_mime_type_pdf() {
        let doc = GenOfficeDocument::new(
            PathBuf::from("report.pdf"),
            DocumentFormat::Pdf,
            "Annual Report",
        );
        assert_eq!(doc.mime_type(), "application/pdf");
    }
    #[test]
    fn suite_register_and_lookup() {
        let mut suite = GenOfficeSuite::new();
        suite.register_app("genoffice-docs");
        assert!(suite.has_app("genoffice-docs"));
        assert!(!suite.has_app("genoffice-sheet"));
    }
    #[test]
    fn format_detected_from_extension() {
        assert_eq!(
            DocumentFormat::from_extension("ODT"),
            Some(DocumentFormat::Odf)
        );
        assert_eq!(
            DocumentFormat::from_path(&PathBuf::from("/docs/report.docx")),
            Some(DocumentFormat::Ooxml)
        );
        assert_eq!(
            DocumentFormat::from_path(&PathBuf::from("/notes/readme.md")),
            Some(DocumentFormat::Markdown)
        );
        assert_eq!(DocumentFormat::from_path(&PathBuf::from("/bin/tool")), None);
        assert_eq!(DocumentFormat::from_extension("xyz"), None);
    }
}

#[cfg(test)]
mod genoffice_extra_tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_mark_offline_ready_aaa() {
        // Arrange
        let doc = GenOfficeDocument::new(PathBuf::from("/doc.odt"), DocumentFormat::Odf, "My Doc");
        assert!(!doc.offline_ready);
        // Act
        let ready = doc.mark_offline_ready();
        // Assert
        assert!(ready.offline_ready);
    }

    #[test]
    fn test_all_mime_types_aaa() {
        // Arrange & Act & Assert
        let p = PathBuf::from("/test");
        assert_eq!(
            GenOfficeDocument::new(p.clone(), DocumentFormat::Pdf, "").mime_type(),
            "application/pdf"
        );
        assert_eq!(
            GenOfficeDocument::new(p.clone(), DocumentFormat::Markdown, "").mime_type(),
            "text/markdown"
        );
        assert_eq!(
            GenOfficeDocument::new(p.clone(), DocumentFormat::Html, "").mime_type(),
            "text/html"
        );
        assert_eq!(
            GenOfficeDocument::new(p.clone(), DocumentFormat::Csv, "").mime_type(),
            "text/csv"
        );
        assert_eq!(
            GenOfficeDocument::new(p, DocumentFormat::Svg, "").mime_type(),
            "image/svg+xml"
        );
    }

    #[test]
    fn test_document_format_equality_aaa() {
        // Arrange
        let f1 = DocumentFormat::Ooxml;
        let f2 = DocumentFormat::Ooxml;
        let f3 = DocumentFormat::Pdf;
        // Act & Assert
        assert_eq!(f1, f2);
        assert_ne!(f1, f3);
    }
}
