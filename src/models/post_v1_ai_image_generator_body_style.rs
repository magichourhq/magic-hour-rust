/// PostV1AiImageGeneratorBodyStyle
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1AiImageGeneratorBodyStyle {
    /// The prompt used for the image.
    pub prompt: String,
}
