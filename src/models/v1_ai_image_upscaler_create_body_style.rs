/// Style settings for the upscale. Use `mode` (`"preserve"`, `"balanced"`, or `"creative"`). Defaults to `"balanced"`.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiImageUpscalerCreateBodyStyle {
    /// Deprecated: use `mode` instead. `"Resemblance"` maps to `"preserve"`. `"Balanced"` and `"Creative"` map to the same-named modes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhancement: Option<
        crate::models::V1AiImageUpscalerCreateBodyStyleEnhancementEnum,
    >,
    /// The upscaling mode. `"preserve"` uses the fast pro pipeline (1× credit multiplier). `"balanced"` and `"creative"` use the creative pipeline (2× credit multiplier). `"pro"` is deprecated and maps to `"preserve"`. Defaults to `"balanced"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<crate::models::V1AiImageUpscalerCreateBodyStyleModeEnum>,
    /// A prompt to guide the final image. Only used when mode is `creative`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
}
