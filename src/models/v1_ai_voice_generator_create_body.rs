/// V1AiVoiceGeneratorCreateBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiVoiceGeneratorCreateBody {
    /// The name of audio. This value is mainly used for your own identification of the audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The content used to generate speech.
    pub style: crate::models::V1AiVoiceGeneratorCreateBodyStyle,
}
