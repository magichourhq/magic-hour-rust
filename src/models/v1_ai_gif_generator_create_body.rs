/// V1AiGifGeneratorCreateBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiGifGeneratorCreateBody {
    /// The name of gif
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub style: crate::models::V1AiGifGeneratorCreateBodyStyle,
}
