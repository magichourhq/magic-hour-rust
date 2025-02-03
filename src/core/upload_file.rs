#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct UploadFile {
    pub file_name: String,
    pub content: bytes::Bytes,
}
impl UploadFile {
    pub fn from_path(path: &str) -> std::io::Result<Self> {
        let path: &std::path::Path = path.as_ref();
        let file_name = path
            .file_name()
            .map(|file_name| file_name.to_string_lossy().into_owned())
            .unwrap_or_default();
        let file_bytes = std::fs::read(path)?;
        Ok(Self {
            file_name,
            content: bytes::Bytes::from(file_bytes),
        })
    }
}
impl From<&UploadFile> for reqwest::multipart::Part {
    fn from(value: &UploadFile) -> Self {
        reqwest::multipart::Part::bytes(value.content.to_vec())
            .file_name(value.file_name.clone())
    }
}
