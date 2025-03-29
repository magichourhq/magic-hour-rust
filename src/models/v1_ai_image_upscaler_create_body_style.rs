/// V1AiImageUpscalerCreateBodyStyle
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiImageUpscalerCreateBodyStyle {
    pub enhancement: crate::models::V1AiImageUpscalerCreateBodyStyleEnhancementEnum,
    /// A prompt to guide the final image. This value is ignored if `enhancement` is not Creative
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
}
