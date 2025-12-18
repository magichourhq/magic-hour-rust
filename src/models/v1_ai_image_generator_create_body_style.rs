/// The art style to use for image generation.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiImageGeneratorCreateBodyStyle {
    /// The prompt used for the image(s).
    pub prompt: String,
    /// Controls the quality of the generated image. Defaults to 'standard' if not specified.
    ///
    /// **Options:**
    /// - `standard` - Standard quality generation. Cost: 5 credits per image.
    /// - `pro` - Pro quality generation with enhanced details and quality. Cost: 30 credits per image.
    ///
    /// Note: Pro mode is available for users on Creator, Pro, or Business tier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_mode: Option<
        crate::models::V1AiImageGeneratorCreateBodyStyleQualityModeEnum,
    >,
    /// The art style to use for image generation. Defaults to 'general' if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool: Option<crate::models::V1AiImageGeneratorCreateBodyStyleToolEnum>,
}
