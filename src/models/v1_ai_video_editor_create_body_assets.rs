/// Provide the assets for video editing.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiVideoEditorCreateBodyAssets {
    /// The video to edit. This value is either
    /// - a direct URL to the file. The URL path must end in a supported file extension.
    /// - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).
    ///
    /// See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details.
    ///
    pub video_file_path: String,
}
