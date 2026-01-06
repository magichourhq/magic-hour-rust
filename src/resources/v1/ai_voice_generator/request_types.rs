/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    /// Give your audio a custom name for easy identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The content used to generate speech.
    pub style: crate::models::V1AiVoiceGeneratorCreateBodyStyle,
}
