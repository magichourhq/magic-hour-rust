/// V1CharacterReplaceCreateBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1CharacterReplaceCreateBody {
    /// Source video and reference character image for the job.
    pub assets: crate::models::V1CharacterReplaceCreateBodyAssets,
    /// End time of your clip (seconds). Must be greater than start_seconds.
    pub end_seconds: f64,
    /// Give your video a custom name for easy identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Output video resolution. Defaults to 480p, the lowest resolution available on your plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<crate::models::V1CharacterReplaceCreateBodyResolutionEnum>,
    /// Start time of your clip (seconds). Must be ≥ 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_seconds: Option<f64>,
    /// Optional style controls for replace vs animate mode and subject selection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<crate::models::V1CharacterReplaceCreateBodyStyle>,
}
