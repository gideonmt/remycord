#[derive(Debug, Clone)]
pub struct AttachedFile {
    pub path: String,
    pub name: String,
}

impl AttachedFile {
    pub fn new(path: impl Into<String>) -> Self {
        let path = path.into();
        let name = std::path::Path::new(&path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&path)
            .to_string();
        
        Self { path, name }
    }
}
