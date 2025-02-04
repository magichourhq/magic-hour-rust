/// V1FilesUploadUrlscreateResponseItemsItem
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1FilesUploadUrlscreateResponseItemsItem {
    /// when the upload url expires, and will need to request a new one.
    pub expires_at: String,
    /// this value is used in APIs that needs assets, such as image_file_path, video_file_path, and audio_file_path
    pub file_path: String,
    /// Used to upload the file to storage, send a PUT request with the file as data to upload.
    pub upload_url: String,
}
