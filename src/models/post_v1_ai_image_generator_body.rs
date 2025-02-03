/// PostV1AiImageGeneratorBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1AiImageGeneratorBody {
    /// number to images to generate
    pub image_count: i64,
    /// The name of image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub orientation: crate::models::PostV1AiImageGeneratorBodyOrientationEnum,
    pub style: crate::models::PostV1AiImageGeneratorBodyStyle,
}
