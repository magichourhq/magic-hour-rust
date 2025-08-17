/// V1PhotoColorizerCreateBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1PhotoColorizerCreateBody {
    /// Provide the assets for photo colorization
    pub assets: crate::models::V1PhotoColorizerCreateBodyAssets,
    /// The name of image. This value is mainly used for your own identification of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
