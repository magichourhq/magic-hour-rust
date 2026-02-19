/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    /// Provide the assets for face swap. For video, The `video_source` field determines whether `video_file_path` or `youtube_url` field is used
    pub assets: crate::models::V1FaceSwapCreateBodyAssets,
    /// End time of your clip (seconds). Must be greater than start_seconds.
    pub end_seconds: f64,
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
    /// Style of the face swap video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<crate::models::V1FaceSwapCreateBodyStyle>,
    /// `width` is deprecated and no longer influences the output video's resolution.
    ///
    /// This field is retained only for backward compatibility and will be removed in a future release.
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub width: crate::core::patch::Patch<i64>,
}
