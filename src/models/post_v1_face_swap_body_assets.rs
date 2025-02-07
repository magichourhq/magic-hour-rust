/// Provide the assets for face swap. For video, The `video_source` field determines whether `video_file_path` or `youtube_url` field is used
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1FaceSwapBodyAssets {
    /// The path of the input image. This value can be either the `file_path` field from the response of the [upload urls API](/docs/api/tag/files/post/v1/files/upload-urls), or the url of the file.
    pub image_file_path: String,
    /// The path of the input video. This field is required if `video_source` is `file`. This value can be either the `file_path` field from the response of the [upload urls API](/docs/api/tag/files/post/v1/files/upload-urls), or the url of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_file_path: Option<String>,
    pub video_source: crate::models::PostV1FaceSwapBodyAssetsVideoSourceEnum,
    /// Using a youtube video as the input source. This field is required if `video_source` is `youtube`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub youtube_url: Option<String>,
}
