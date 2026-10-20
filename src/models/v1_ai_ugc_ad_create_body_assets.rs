/// V1AiUgcAdCreateBodyAssets
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiUgcAdCreateBodyAssets {
    /// Prepared image to animate. This value is either
    /// - a direct URL to the video file
    /// - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).
    ///
    /// See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details.
    ///
    pub image_file_path: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
