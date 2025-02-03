///
/// * `custom` - use your own prompt for the video.
/// * `use_lyrics` - Use the lyrics of the audio to create the prompt. If this option is selected, then `assets.audio_source` must be `file` or `youtube`.
/// * `ai_choose` - Let AI write the prompt. If this option is selected, then `assets.audio_source` must be `file` or `youtube`.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum PostV1AnimationBodyStylePromptTypeEnum {
    #[default]
    #[serde(rename = "ai_choose")]
    AiChoose,
    #[serde(rename = "custom")]
    Custom,
    #[serde(rename = "use_lyrics")]
    UseLyrics,
}
impl std::fmt::Display for PostV1AnimationBodyStylePromptTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostV1AnimationBodyStylePromptTypeEnum::AiChoose => "ai_choose",
            PostV1AnimationBodyStylePromptTypeEnum::Custom => "custom",
            PostV1AnimationBodyStylePromptTypeEnum::UseLyrics => "use_lyrics",
        };
        write!(f, "{}", str_val)
    }
}
