/// Provide the audio file and an optional reference image.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AudioToVideoCreateBodyAssets {
    /// The path of the audio file. This value is either
    /// - a direct URL to the video file
    /// - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).
    ///
    /// See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details.
    ///
    pub audio_file_path: String,
    /// Reference image for the initial frame of the video. This value is either
    /// - a direct URL to the video file
    /// - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).
    ///
    /// See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_file_path: Option<String>,
}
