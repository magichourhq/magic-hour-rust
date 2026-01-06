/// V1ImageBackgroundRemoverCreateBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1ImageBackgroundRemoverCreateBody {
    /// Provide the assets for background removal
    pub assets: crate::models::V1ImageBackgroundRemoverCreateBodyAssets,
    /// Give your image a custom name for easy identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
