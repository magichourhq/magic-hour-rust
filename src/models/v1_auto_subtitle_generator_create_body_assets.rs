/// Provide the assets for auto subtitle generator
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AutoSubtitleGeneratorCreateBodyAssets {
    /// This is the video used to add subtitles. This value can be either the `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls), or the url of the file.
    pub video_file_path: String,
}
