/// The content used to generate speech.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiVoiceGeneratorCreateBodyStyle {
    /// Spoken language. Use a canonical lowercase language name, or auto to detect the language from the text. Defaults to auto.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<crate::models::V1AiVoiceGeneratorCreateBodyStyleLanguageEnum>,
    /// Text used to generate speech. The character limit is 5000 characters.
    pub prompt: String,
    /// Speech speed multiplier, from 0.75 to 1.30 inclusive. Defaults to 1 (normal speed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
    /// The voice to use for the speech. Use the `id` of a voice returned by `GET /v1/voices`.
    pub voice_name: crate::models::V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum,
}
