/// V1AiImageGeneratorCreateBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiImageGeneratorCreateBody {
    /// Number of images to generate.
    pub image_count: i64,
    /// The name of image. This value is mainly used for your own identification of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The orientation of the output image(s).
    pub orientation: crate::models::V1AiImageGeneratorCreateBodyOrientationEnum,
    /// The art style to use for image generation.
    pub style: crate::models::V1AiImageGeneratorCreateBodyStyle,
}
