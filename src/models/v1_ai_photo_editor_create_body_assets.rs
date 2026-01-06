/// Provide the assets for photo editor
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiPhotoEditorCreateBodyAssets {
    /// The image used to generate the output. This value is either
    /// - a direct URL to the video file
    /// - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).
    ///
    /// See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details.
    ///
    pub image_file_path: String,
}
