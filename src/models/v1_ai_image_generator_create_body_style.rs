/// The art style to use for image generation.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiImageGeneratorCreateBodyStyle {
    /// The prompt used for the image(s).
    pub prompt: String,
    /// DEPRECATED: Use `model` field instead for explicit model selection.
    ///
    /// Legacy quality mode mapping:
    /// - `standard` → `z-image-turbo` model
    /// - `pro` → `seedream` model
    ///
    /// If model is specified, it will take precedence over the legacy quality_mode field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_mode: Option<
        crate::models::V1AiImageGeneratorCreateBodyStyleQualityModeEnum,
    >,
    /// The art style to use for image generation. Defaults to 'general' if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool: Option<crate::models::V1AiImageGeneratorCreateBodyStyleToolEnum>,
}
