#[derive(Debug, Clone)]
pub struct FileAssociation {
    pub mime_type: String,
    pub app_id: String,
}

impl FileAssociation {
    pub fn new(mime: impl Into<String>, app: impl Into<String>) -> Self {
        Self { mime_type: mime.into(), app_id: app.into() }
    }
}

#[derive(Debug, Default)]
pub struct OfficeIntegration {
    associations: Vec<FileAssociation>,
}

impl OfficeIntegration {
    pub fn new() -> Self { Self::default() }
    pub fn register(&mut self, assoc: FileAssociation) { self.associations.push(assoc); }
    pub fn app_for_mime(&self, mime: &str) -> Option<&str> {
        self.associations.iter().find(|a| a.mime_type == mime).map(|a| a.app_id.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lookup() {
        let mut oi = OfficeIntegration::new();
        oi.register(FileAssociation::new("application/pdf", "genoffice-pdf"));
        assert_eq!(oi.app_for_mime("application/pdf"), Some("genoffice-pdf"));
    }
}
