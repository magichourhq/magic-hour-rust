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
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiImageGeneratorCreateBodyModelEnum {
    #[default]
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "flux-schnell")]
    FluxSchnell,
    #[serde(rename = "nano-banana-pro")]
    NanoBananaPro,
    #[serde(rename = "seedream")]
    Seedream,
    #[serde(rename = "z-image-turbo")]
    ZImageTurbo,
}
impl std::fmt::Display for V1AiImageGeneratorCreateBodyModelEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiImageGeneratorCreateBodyModelEnum::Default => "default",
            V1AiImageGeneratorCreateBodyModelEnum::FluxSchnell => "flux-schnell",
            V1AiImageGeneratorCreateBodyModelEnum::NanoBananaPro => "nano-banana-pro",
            V1AiImageGeneratorCreateBodyModelEnum::Seedream => "seedream",
            V1AiImageGeneratorCreateBodyModelEnum::ZImageTurbo => "z-image-turbo",
        };
        write!(f, "{}", str_val)
    }
}
