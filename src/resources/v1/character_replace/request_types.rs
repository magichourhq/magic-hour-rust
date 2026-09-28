/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    /// Source video and reference character image for the job.
    pub assets: crate::models::V1CharacterReplaceCreateBodyAssets,
    /// End time of your clip (seconds). Must be greater than start_seconds.
    pub end_seconds: f64,
    /// Model to use. Defaults to `wan-animate`.
    ///
    /// * **`wan-animate`**: 480p, 720p. Supports `points` subject selection.
    /// * **`kling-3.0`**: 720p, 1080p. Clips of 3–10 seconds in `replace` mode or 3–30 seconds in `animate` mode. Picks the main person automatically, so `points` are rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<crate::models::V1CharacterReplaceCreateBodyModelEnum>,
    /// Give your video a custom name for easy identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Output video resolution. Must be supported by `model`. Defaults to the lowest resolution available on your plan for that model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<crate::models::V1CharacterReplaceCreateBodyResolutionEnum>,
    /// Start time of your clip (seconds). Must be ≥ 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_seconds: Option<f64>,
    /// Optional style controls for replace vs animate mode and subject selection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<crate::models::V1CharacterReplaceCreateBodyStyle>,
}
