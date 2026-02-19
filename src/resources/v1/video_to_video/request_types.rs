/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    /// Provide the assets for video-to-video. For video, The `video_source` field determines whether `video_file_path` or `youtube_url` field is used
    pub assets: crate::models::V1VideoToVideoCreateBodyAssets,
    /// End time of your clip (seconds). Must be greater than start_seconds.
    pub end_seconds: f64,
    /// Determines whether the resulting video will have the same frame per second as the original video, or half.
    /// * `FULL` - the result video will have the same FPS as the input video
    /// * `HALF` - the result video will have half the FPS as the input video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fps_resolution: Option<crate::models::V1VideoToVideoCreateBodyFpsResolutionEnum>,
    /// `height` is deprecated and no longer influences the output video's resolution.
    ///
    /// This field is retained only for backward compatibility and will be removed in a future release.
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub height: crate::core::patch::Patch<i64>,
    /// Give your video a custom name for easy identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Start time of your clip (seconds). Must be ≥ 0.
    pub start_seconds: f64,
    pub style: crate::models::V1VideoToVideoCreateBodyStyle,
    /// `width` is deprecated and no longer influences the output video's resolution.
    ///
    /// This field is retained only for backward compatibility and will be removed in a future release.
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub width: crate::core::patch::Patch<i64>,
}
