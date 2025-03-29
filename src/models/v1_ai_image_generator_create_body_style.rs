/// V1AiImageGeneratorCreateBodyStyle
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiImageGeneratorCreateBodyStyle {
    /// The prompt used for the image.
    pub prompt: String,
}
