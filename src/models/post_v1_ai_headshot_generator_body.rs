/// PostV1AiHeadshotGeneratorBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1AiHeadshotGeneratorBody {
    /// Provide the assets for headshot photo
    pub assets: crate::models::PostV1AiHeadshotGeneratorBodyAssets,
    /// The name of image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<crate::models::PostV1AiHeadshotGeneratorBodyStyle>,
}
