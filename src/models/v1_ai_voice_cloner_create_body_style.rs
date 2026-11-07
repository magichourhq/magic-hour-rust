/// V1AiVoiceClonerCreateBodyStyle
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiVoiceClonerCreateBodyStyle {
    /// Spoken language. Use a canonical lowercase language name, or auto to detect the language from the text. Defaults to auto.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<crate::models::V1AiVoiceClonerCreateBodyStyleLanguageEnum>,
    /// Text used to generate speech from the cloned voice. The character limit is 5000 characters.
    pub prompt: String,
    /// Speech speed multiplier, from 0.75 to 1.30 inclusive. Defaults to 1 (normal speed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
}
