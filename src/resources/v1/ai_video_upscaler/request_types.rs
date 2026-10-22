/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    /// Source video for the upscale job.
    pub assets: crate::models::V1AiVideoUpscalerCreateBodyAssets,
    /// End time of your clip (seconds). Must be greater than start_seconds.
    pub end_seconds: f64,
    /// Give your video a custom name for easy identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Output resolution. `720p` and `1080p` set the output's short side, `2k` and `4k` its long side; the aspect ratio is kept. Requires a paid plan.
    pub resolution: crate::models::V1AiVideoUpscalerCreateBodyResolutionEnum,
    /// Start time of your clip (seconds). Must be ≥ 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_seconds: Option<f64>,
}
