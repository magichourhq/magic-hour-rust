/// V1AiMusicGeneratorCreateBodyStyle
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiMusicGeneratorCreateBodyStyle {
    /// Generate music without vocals. When true, lyrics is ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrumental: Option<bool>,
    /// Custom lyrics. Omit or leave blank for AI-generated lyrics when instrumental is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lyrics: Option<String>,
    /// Describe the song, including genre, mood, and instruments.
    pub prompt: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
