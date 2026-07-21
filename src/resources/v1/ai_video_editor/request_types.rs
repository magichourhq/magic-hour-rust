/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    /// Provide the assets for video editing.
    pub assets: crate::models::V1AiVideoEditorCreateBodyAssets,
    /// End time of your clip in seconds. Must be greater than `start_seconds`. Duration must be between 3 and 10 seconds.
    pub end_seconds: f64,
    /// Editing model. Defaults to `ltx-2.3` for free tier and `gemini-omni` for paid. Use `ltx-2.3` for LTX video edit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<crate::models::V1AiVideoEditorCreateBodyModelEnum>,
    /// Give your video a custom name for easy identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Output resolution. Defaults to `480p` for free tier and `720p` for paid. Google Omni supports 720p only; LTX-2.3 supports 480p, 720p, and 1080p.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<crate::models::V1AiVideoEditorCreateBodyResolutionEnum>,
    /// Start time of your clip (seconds). Must be ≥ 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_seconds: Option<f64>,
    pub style: crate::models::V1AiVideoEditorCreateBodyStyle,
}
