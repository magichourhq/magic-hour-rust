/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    /// Provide the assets for background removal
    pub assets: crate::models::V1ImageBackgroundRemoverCreateBodyAssets,
    /// The name of image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
