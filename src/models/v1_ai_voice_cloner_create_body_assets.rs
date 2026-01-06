/// Provide the assets for voice cloning.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiVoiceClonerCreateBodyAssets {
    /// The audio used to clone the voice. This value is either
    /// - a direct URL to the video file
    /// - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).
    ///
    /// See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details.
    ///
    pub audio_file_path: String,
}
