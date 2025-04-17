/// V1AiMemeGeneratorCreateBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiMemeGeneratorCreateBody {
    /// The name of the meme.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub style: crate::models::V1AiMemeGeneratorCreateBodyStyle,
}
