/// Provide the assets for video-to-video. For video, The `video_source` field determines whether `video_file_path` or `youtube_url` field is used
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1VideoToVideoCreateBodyAssets {
    /// Required if `video_source` is `file`. This value is either
    /// - a direct URL to the video file
    /// - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).
    ///
    /// Please refer to the [Input File documentation](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) to learn more.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_file_path: Option<String>,
    pub video_source: crate::models::V1VideoToVideoCreateBodyAssetsVideoSourceEnum,
    /// Using a youtube video as the input source. This field is required if `video_source` is `youtube`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub youtube_url: Option<String>,
}
