/// V1AiClothesChangerCreateBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiClothesChangerCreateBody {
    /// Provide the assets for clothes changer
    pub assets: crate::models::V1AiClothesChangerCreateBodyAssets,
    /// The name of image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
