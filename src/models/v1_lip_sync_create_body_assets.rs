/// Provide the assets for lip-sync. For video, The `video_source` field determines whether `video_file_path` or `youtube_url` field is used
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1LipSyncCreateBodyAssets {
    /// The path of the audio file. This value can be either the `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls), or the url of the file.
    pub audio_file_path: String,
    /// The path of the input video. This field is required if `video_source` is `file`. This value can be either the `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls), or the url of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_file_path: Option<String>,
    pub video_source: crate::models::V1LipSyncCreateBodyAssetsVideoSourceEnum,
    /// Using a youtube video as the input source. This field is required if `video_source` is `youtube`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub youtube_url: Option<String>,
}
