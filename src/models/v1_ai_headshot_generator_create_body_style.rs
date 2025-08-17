/// V1AiHeadshotGeneratorCreateBodyStyle
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiHeadshotGeneratorCreateBodyStyle {
    /// Prompt used to guide the style of your headshot. We recommend omitting the prompt unless you want to customize your headshot. You can visit [AI headshot generator](https://magichour.ai/create/ai-headshot-generator) to view an example of a good prompt used for our 'Professional' style.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
}
