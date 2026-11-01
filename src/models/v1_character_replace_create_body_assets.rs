/// Source video and reference character image for the job.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1CharacterReplaceCreateBodyAssets {
    /// Reference character image used as the replacement or animation target. This value is either
    /// - a direct URL to the file. The URL path must end in a supported file extension.
    /// - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).
    ///
    /// See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details.
    ///
    pub image_file_path: String,
    /// Source video containing the subject to replace or animate. This value is either
    /// - a direct URL to the file. The URL path must end in a supported file extension.
    /// - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).
    ///
    /// See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details.
    ///
    pub video_file_path: String,
}
