/// V1AiVideoEditorCreateBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiVideoEditorCreateBody {
    /// Provide the assets for video editing.
    pub assets: crate::models::V1AiVideoEditorCreateBodyAssets,
    /// End time of your clip in seconds. Must be greater than `start_seconds`. Allowed duration depends on model: `gemini-omni-1.1`: 3-10s, `wan-3.0`: 2-15s, `ltx-2.5`: 0.5-60s (30s at 1080p).
    pub end_seconds: f64,
    /// Editing model. Defaults to LTX 2.5 for free tier and `gemini-omni-1.1` for paid. `wan-3.0` requires a paid tier. `gemini-omni` is deprecated; use `gemini-omni-1.1` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<crate::models::V1AiVideoEditorCreateBodyModelEnum>,
    /// Give your video a custom name for easy identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Output resolution. Defaults to `480p` for free tier and `720p` for paid. Supported per model: `gemini-omni-1.1`: 720p, 1080p; `wan-3.0`: 480p, 720p, 1080p; `ltx-2.5`: 480p, 720p, 1080p.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<crate::models::V1AiVideoEditorCreateBodyResolutionEnum>,
    /// Start time of your clip (seconds). Must be ≥ 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_seconds: Option<f64>,
    pub style: crate::models::V1AiVideoEditorCreateBodyStyle,
}
