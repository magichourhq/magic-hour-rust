/// ListRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListRequest {
    /// The create endpoint to price, named by its path. For example `ai-voice-generator` for `POST /v1/ai-voice-generator`.
    pub tool: crate::models::V1EstimateAudioListToolEnum,
    /// The number of characters in the `style.prompt` of the create request. Used by `ai-voice-generator`, `ai-voice-cloner`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_count: Option<i64>,
    /// The `duration` of the create request. Used by `ai-music-generator`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
}
