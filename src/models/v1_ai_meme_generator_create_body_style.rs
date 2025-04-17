/// V1AiMemeGeneratorCreateBodyStyle
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiMemeGeneratorCreateBodyStyle {
    /// Whether to search the web for meme content.
    #[serde(rename = "searchWeb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_web: Option<bool>,
    /// To use our templates, pass in one of the enum values.
    pub template: crate::models::V1AiMemeGeneratorCreateBodyStyleTemplateEnum,
    /// The topic of the meme.
    pub topic: String,
}
