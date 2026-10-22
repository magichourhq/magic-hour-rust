/// V1AiMusicGeneratorCreateBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiMusicGeneratorCreateBody {
    /// Requested track duration in seconds, from 15 to 300. Determines credits charged.
    pub duration: i64,
    /// Give your audio a custom name for easy identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub style: crate::models::V1AiMusicGeneratorCreateBodyStyle,
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
