/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    /// Provide the assets for video editing.
    pub assets: crate::models::V1AiVideoEditorCreateBodyAssets,
    /// End time of your clip in seconds. Must be greater than `start_seconds`. Duration must be between 3 and 10 seconds.
    pub end_seconds: f64,
    /// Give your video a custom name for easy identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Start time of your clip (seconds). Must be ≥ 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_seconds: Option<f64>,
    pub style: crate::models::V1AiVideoEditorCreateBodyStyle,
}
