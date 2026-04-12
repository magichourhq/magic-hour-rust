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
    /// - `flux-schnell` - from 5 credits/image
    ///   - Supported resolutions: 640px, 1k, 2k
    ///   - Available for tiers: free, creator, pro, business
    ///   - Image count allowed: 1, 2, 3, 4
    /// - `z-image-turbo` - from 5 credits/image
    ///   - Supported resolutions: 640px, 1k, 2k
    ///   - Available for tiers: free, creator, pro, business
    ///   - Image count allowed: 1, 2, 3, 4
    /// - `seedream-v4` - from 40 credits/image
    ///   - Supported resolutions: 640px, 1k, 2k, 4k
    ///   - Available for tiers: free, creator, pro, business
    ///   - Image count allowed: 1, 2, 3, 4
    /// - `nano-banana` - from 50 credits/image
    ///   - Supported resolutions: 640px, 1k
    ///   - Available for tiers: free, creator, pro, business
    ///   - Image count allowed: 1, 2, 3, 4
    /// - `nano-banana-2` - from 100 credits/image
    ///   - Supported resolutions: 640px, 1k, 2k, 4k
    ///   - Available for tiers: free, creator, pro, business
    ///   - Image count allowed: 1, 2, 3, 4
    /// - `nano-banana-pro` - from 150 credits/image
    ///   - Supported resolutions: 1k, 2k, 4k
    ///   - Available for tiers: creator, pro, business
    ///   - Image count allowed: 1, 4, 9, 16
    ///
    /// **Deprecated Enum Values:**
    /// - `seedream` - Use `seedream-v4` instead.
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
    /// Maximum resolution (longest edge) for the output image.
    ///
    /// **Options:**
    /// - `640px` — up to 640px
    /// - `1k` — up to 1024px
    /// - `2k` — up to 2048px
    /// - `4k` — up to 4096px
    /// - `auto` — **Deprecated.** Mapped server-side from your subscription tier to the best matching resolution the model supports
    ///
    /// **Per-model support:**
    /// - `flux-schnell` - 640px, 1k, 2k
    /// - `z-image-turbo` - 640px, 1k, 2k
    /// - `seedream-v4` - 640px, 1k, 2k, 4k
    /// - `nano-banana` - 640px, 1k
    /// - `nano-banana-2` - 640px, 1k, 2k, 4k
    /// - `nano-banana-pro` - 1k, 2k, 4k
    ///
    /// Note: Resolution availability depends on the model and your subscription tier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<crate::models::V1AiImageGeneratorCreateBodyResolutionEnum>,
    /// The art style to use for image generation.
    pub style: crate::models::V1AiImageGeneratorCreateBodyStyle,
}
