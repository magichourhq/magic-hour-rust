/// PostV1AiImageUpscalerBodyStyle
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1AiImageUpscalerBodyStyle {
    pub enhancement: crate::models::PostV1AiImageUpscalerBodyStyleEnhancementEnum,
    /// A prompt to guide the final image. This value is ignored if `enhancement` is not Creative
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
}
