/// Style settings for the upscale. Use `mode` to select between `"pro"` (faster, no enhancement required) and `"creative"` (defaults to `"Balanced"` enhancement). Defaults to `"creative"`.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiImageUpscalerCreateBodyStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhancement: Option<
        crate::models::V1AiImageUpscalerCreateBodyStyleEnhancementEnum,
    >,
    /// The upscaling mode. `"pro"` is faster and does not require `enhancement`. `"creative"` requires `enhancement`. Defaults to `"creative"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<crate::models::V1AiImageUpscalerCreateBodyStyleModeEnum>,
    /// A prompt to guide the final image. This value is ignored if `enhancement` is not Creative
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
}
