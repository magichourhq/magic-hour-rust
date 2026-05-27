/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    /// Provide the audio file and an optional reference image.
    pub assets: crate::models::V1AudioToVideoCreateBodyAssets,
    /// End time of your clip (seconds). Must be greater than start_seconds.
    pub end_seconds: f64,
    /// Give your video a custom name for easy identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Output video resolution. Defaults to `720p` on paid tiers and `480p` on free tiers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<crate::models::V1AudioToVideoCreateBodyResolutionEnum>,
    /// Start time of your clip (seconds). Must be ≥ 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_seconds: Option<f64>,
    /// Attributes used to dictate the style of the output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<crate::models::V1AudioToVideoCreateBodyStyle>,
}
