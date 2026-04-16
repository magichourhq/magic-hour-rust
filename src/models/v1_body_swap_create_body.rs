/// V1BodySwapCreateBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1BodySwapCreateBody {
    /// Person image and scene image for body swap
    pub assets: crate::models::V1BodySwapCreateBodyAssets,
    /// Give your image a custom name for easy identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Output resolution. Determines credits charged for the run.
    pub resolution: crate::models::V1BodySwapCreateBodyResolutionEnum,
}
