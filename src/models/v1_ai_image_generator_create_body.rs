/// V1AiImageGeneratorCreateBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiImageGeneratorCreateBody {
    /// number to images to generate
    pub image_count: i64,
    /// The name of image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub orientation: crate::models::V1AiImageGeneratorCreateBodyOrientationEnum,
    pub style: crate::models::V1AiImageGeneratorCreateBodyStyle,
}
