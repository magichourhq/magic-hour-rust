/// V1AiHeadshotGeneratorCreateBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiHeadshotGeneratorCreateBody {
    /// Provide the assets for headshot photo
    pub assets: crate::models::V1AiHeadshotGeneratorCreateBodyAssets,
    /// The name of image. This value is mainly used for your own identification of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<crate::models::V1AiHeadshotGeneratorCreateBodyStyle>,
}
