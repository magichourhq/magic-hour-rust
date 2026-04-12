/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    /// The aspect ratio of the output image(s). If not specified, defaults to `auto`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<crate::models::V1AiImageEditorCreateBodyAspectRatioEnum>,
    /// Provide the assets for image edit
    pub assets: crate::models::V1AiImageEditorCreateBodyAssets,
    /// Number of images to generate. Maximum varies by model. Defaults to 1 if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_count: Option<f64>,
    /// The AI model to use for image editing. Each model has different capabilities and costs.
    ///
    /// **Models:**
    /// - `default` - Use the model we recommend, which will change over time. This is recommended unless you need a specific model. This is the default behavior.
    /// - `qwen-edit` - from 10 credits/image
    ///   - Supported resolutions: 640px, 1k, 2k
    ///   - Available for tiers: free, creator, pro, business
    ///   - Image count allowed: 1, 2, 3, 4
    ///   - Max additional input images: 2
    /// - `nano-banana` - from 50 credits/image
    ///   - Supported resolutions: 640px, 1k
    ///   - Available for tiers: free, creator, pro, business
    ///   - Image count allowed: 1, 2, 3, 4
    ///   - Max additional input images: 9
    /// - `nano-banana-2` - from 100 credits/image
    ///   - Supported resolutions: 640px, 1k, 2k, 4k
    ///   - Available for tiers: free, creator, pro, business
    ///   - Image count allowed: 1, 2, 3, 4
    ///   - Max additional input images: 9
    /// - `seedream-v4` - from 40 credits/image
    ///   - Supported resolutions: 640px, 1k, 2k, 4k
    ///   - Available for tiers: free, creator, pro, business
    ///   - Image count allowed: 1, 2, 3, 4
    ///   - Max additional input images: 9
    /// - `nano-banana-pro` - from 150 credits/image
    ///   - Supported resolutions: 1k, 2k, 4k
    ///   - Available for tiers: creator, pro, business
    ///   - Image count allowed: 1, 4, 9, 16
    ///   - Max additional input images: 9
    /// - `seedream-v4.5` - from 50 credits/image
    ///   - Supported resolutions: 640px, 1k, 2k, 4k
    ///   - Available for tiers: creator, pro, business
    ///   - Image count allowed: 1, 2, 3, 4
    ///   - Max additional input images: 9
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<crate::models::V1AiImageEditorCreateBodyModelEnum>,
    /// Give your image a custom name for easy identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
    /// - `qwen-edit` - 640px, 1k, 2k
    /// - `nano-banana` - 640px, 1k
    /// - `nano-banana-2` - 640px, 1k, 2k, 4k
    /// - `seedream-v4` - 640px, 1k, 2k, 4k
    /// - `nano-banana-pro` - 1k, 2k, 4k
    /// - `seedream-v4.5` - 640px, 1k, 2k, 4k
    ///
    /// Note: Resolution availability depends on the model and your subscription tier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<crate::models::V1AiImageEditorCreateBodyResolutionEnum>,
    pub style: crate::models::V1AiImageEditorCreateBodyStyle,
}
