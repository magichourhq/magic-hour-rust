/// PostV1AiHeadshotGeneratorBodyStyle
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1AiHeadshotGeneratorBodyStyle {
    /// A prompt to guide the final image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
}
