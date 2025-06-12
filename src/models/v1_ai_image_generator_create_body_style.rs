/// V1AiImageGeneratorCreateBodyStyle
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiImageGeneratorCreateBodyStyle {
    /// The prompt used for the image.
    pub prompt: String,
    /// The art style to use for image generation. Defaults to 'general' if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool: Option<crate::models::V1AiImageGeneratorCreateBodyStyleToolEnum>,
}
