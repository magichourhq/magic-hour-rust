/// PostV1AiClothesChangerBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1AiClothesChangerBody {
    /// Provide the assets for clothes changer
    pub assets: crate::models::PostV1AiClothesChangerBodyAssets,
    /// The name of image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
