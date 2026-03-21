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
    /// - `qwen-edit` - 10 credits/image
    ///   - Available for tiers: free, creator, pro, business
    ///   - Image count allowed: 1
    ///   - Max additional input images: 2
    /// - `nano-banana` - 50 credits/image
    ///   - Available for tiers: free, creator, pro, business
    ///   - Image count allowed: 1
    ///   - Max additional input images: 9
    /// - `nano-banana-2` - 100 credits/image
    ///   - Available for tiers: free, creator, pro, business
    ///   - Image count allowed: 1
    ///   - Max additional input images: 9
    /// - `seedream-v4` - 50 credits/image
    ///   - Available for tiers: free, creator, pro, business
    ///   - Image count allowed: 1
    ///   - Max additional input images: 9
    /// - `nano-banana-pro` - 150 credits/image
    ///   - Available for tiers: creator, pro, business
    ///   - Image count allowed: 1, 4, 9, 16
    ///   - Max additional input images: 9
    /// - `seedream-v4.5` - 100 credits/image
    ///   - Available for tiers: creator, pro, business
    ///   - Image count allowed: 1
    ///   - Max additional input images: 9
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<crate::models::V1AiImageEditorCreateBodyModelEnum>,
    /// Give your image a custom name for easy identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Maximum resolution for the generated image.
    ///
    /// **Options:**
    /// - `auto` - Automatic resolution (all tiers, default)
    /// - `2k` - Up to 2048px (requires Pro or Business tier)
    /// - `4k` - Up to 4096px (requires Business tier)
    ///
    /// Note: Resolution availability depends on your subscription tier. Defaults to `auto` if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<crate::models::V1AiImageEditorCreateBodyResolutionEnum>,
    pub style: crate::models::V1AiImageEditorCreateBodyStyle,
}
