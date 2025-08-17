/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    /// Provide the assets for lip-sync. For video, The `video_source` field determines whether `video_file_path` or `youtube_url` field is used
    pub assets: crate::models::V1LipSyncCreateBodyAssets,
    /// The end time of the input video in seconds. This value is used to trim the input video. The value must be greater than 0.1, and more than the start_seconds.
    pub end_seconds: f64,
    /// `height` is deprecated and no longer influences the output video's resolution.
    ///
    /// Output resolution is determined by the **minimum** of:
    /// - The resolution of the input video
    /// - The maximum resolution allowed by your subscription tier. See our [pricing page](https://magichour.ai/pricing) for more details.
    ///
    /// This field is retained only for backward compatibility and will be removed in a future release.
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub height: crate::core::patch::Patch<i64>,
    /// Defines the maximum FPS (frames per second) for the output video. If the input video's FPS is lower than this limit, the output video will retain the input FPS. This is useful for reducing unnecessary frame usage in scenarios where high FPS is not required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fps_limit: Option<f64>,
    /// The name of video. This value is mainly used for your own identification of the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The start time of the input video in seconds. This value is used to trim the input video. The value must be greater than 0.
    pub start_seconds: f64,
    /// `width` is deprecated and no longer influences the output video's resolution.
    ///
    /// Output resolution is determined by the **minimum** of:
    /// - The resolution of the input video
    /// - The maximum resolution allowed by your subscription tier. See our [pricing page](https://magichour.ai/pricing) for more details.
    ///
    /// This field is retained only for backward compatibility and will be removed in a future release.
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub width: crate::core::patch::Patch<i64>,
}
