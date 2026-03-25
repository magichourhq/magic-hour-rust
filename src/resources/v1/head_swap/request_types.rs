/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    /// Provide the body and head images for head swap
    pub assets: crate::models::V1HeadSwapCreateBodyAssets,
    /// Constrains the larger dimension (height or width) of the output. Omit to use the maximum allowed for your plan (capped at 2048px). Values above your plan maximum are clamped down to your plan's maximum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_resolution: Option<i64>,
    /// Give your image a custom name for easy identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
