/// V1AiHeadshotGeneratorCreateBodyStyle
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiHeadshotGeneratorCreateBodyStyle {
    /// A prompt to guide the final image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
}
