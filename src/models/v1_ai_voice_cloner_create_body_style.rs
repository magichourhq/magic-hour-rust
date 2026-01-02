/// V1AiVoiceClonerCreateBodyStyle
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiVoiceClonerCreateBodyStyle {
    /// Text used to generate speech from the cloned voice. The character limit is 1000 characters.
    pub prompt: String,
}
