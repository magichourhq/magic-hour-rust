/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    /// The aspect ratio of the output image(s). If not specified, defaults to `1:1` (square).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<crate::models::V1AiImageGeneratorCreateBodyAspectRatioEnum>,
    /// Number of images to generate. Maximum varies by model.
    pub image_count: i64,
    /// The AI model to use for image generation. Each model has different capabilities and costs.
    ///
    /// **Models:**
    /// - `default` - Use the model we recommend, which will change over time. This is recommended unless you need a specific model. This is the default behavior.
    /// - `flux-schnell` - 5 credits/image
    ///   - Supported resolutions: auto
    ///   - Available for tiers: free, creator, pro, business
    ///   - Image count allowed: 1, 2, 3, 4
    /// - `z-image-turbo` - 5 credits/image
    ///   - Supported resolutions: auto, 2k
    ///   - Available for tiers: free, creator, pro, business
    ///   - Image count allowed: 1, 2, 3, 4
    /// - `seedream` - 30 credits/image
    ///   - Supported resolutions: auto, 2k, 4k
    ///   - Available for tiers: free, creator, pro, business
    ///   - Image count allowed: 1, 2, 3, 4
    /// - `nano-banana-pro` - 150 credits/image
    ///   - Supported resolutions: auto
    ///   - Available for tiers: creator, pro, business
    ///   - Image count allowed: 1, 4, 9, 16
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<crate::models::V1AiImageGeneratorCreateBodyModelEnum>,
    /// Give your image a custom name for easy identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// DEPRECATED: Use `aspect_ratio` instead.
    ///
    /// The orientation of the output image(s). `aspect_ratio` takes precedence when `orientation` if both are provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<crate::models::V1AiImageGeneratorCreateBodyOrientationEnum>,
    /// Maximum resolution for the generated image.
    ///
    /// **Options:**
    /// - `auto` - Automatic resolution (all tiers, default)
    /// - `2k` - Up to 2048px (requires Pro or Business tier)
    /// - `4k` - Up to 4096px (requires Business tier)
    ///
    /// Note: Resolution availability depends on the model and your subscription tier. See `model` field for which resolutions each model supports. Defaults to `auto` if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<crate::models::V1AiImageGeneratorCreateBodyResolutionEnum>,
    /// The art style to use for image generation.
    pub style: crate::models::V1AiImageGeneratorCreateBodyStyle,
}
