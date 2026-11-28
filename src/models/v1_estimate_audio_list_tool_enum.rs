/// The create endpoint to price, named by its path. For example `ai-voice-generator` for `POST /v1/ai-voice-generator`.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1EstimateAudioListToolEnum {
    #[default]
    #[serde(rename = "ai-music-generator")]
    AiMusicGenerator,
    #[serde(rename = "ai-voice-cloner")]
    AiVoiceCloner,
    #[serde(rename = "ai-voice-generator")]
    AiVoiceGenerator,
}
impl std::fmt::Display for V1EstimateAudioListToolEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1EstimateAudioListToolEnum::AiMusicGenerator => "ai-music-generator",
            V1EstimateAudioListToolEnum::AiVoiceCloner => "ai-voice-cloner",
            V1EstimateAudioListToolEnum::AiVoiceGenerator => "ai-voice-generator",
        };
        write!(f, "{}", str_val)
    }
}
