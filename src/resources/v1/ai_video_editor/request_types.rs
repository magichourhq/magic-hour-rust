/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    /// Provide the assets for video editing.
    pub assets: crate::models::V1AiVideoEditorCreateBodyAssets,
    /// End time of your clip in seconds. Must be greater than `start_seconds`. Allowed duration depends on model: `gemini-omni-1.2`: 3-15s, `wan-3.0`: 2-15s, `ltx-2.5`: 0.5-60s (30s at 1080p), `kling-4.0`: 3-30s.
    pub end_seconds: f64,
    /// The AI model to use for video editing. Defaults to `ltx-2.5` on free tiers and `gemini-omni-1.2` on paid tiers.
    ///
    /// * `gemini-omni-1.2`: Best for precise clips up to 15s, references, and high-resolution output. Requires a paid tier.
    /// * `wan-3.0`: High-quality video with native audio, long clips, and end-frame control. Requires a paid tier.
    /// * `ltx-2.5`: Fastest for general scenes, long clips, audio, and rapid iteration.
    /// * `kling-4.0`: Create cinematic videos with image and video references and optional audio. Requires a paid tier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<crate::models::V1AiVideoEditorCreateBodyModelEnum>,
    /// Give your video a custom name for easy identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Name references without @ and refer to them as @name in style.prompt. Names must be unique, ignoring case. Maximum 10 references total. Omit for no references.
    ///
    /// Reference images for gemini-omni-1.2, wan-3.0, kling-4.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<Vec<crate::models::V1AiVideoEditorCreateBodyReferencesItem>>,
    /// Output resolution. Defaults to `480p` for free tier and `720p` for paid. Supported per model: `gemini-omni-1.2`: 360p, 720p, 1080p, 4k; `wan-3.0`: 480p, 720p, 1080p; `ltx-2.5`: 480p, 720p, 1080p; `kling-4.0`: 720p, 1080p.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<crate::models::V1AiVideoEditorCreateBodyResolutionEnum>,
    /// Start time of your clip (seconds). Must be ≥ 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_seconds: Option<f64>,
    pub style: crate::models::V1AiVideoEditorCreateBodyStyle,
}
