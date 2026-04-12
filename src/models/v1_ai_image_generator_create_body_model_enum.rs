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
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiImageGeneratorCreateBodyModelEnum {
    #[default]
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "flux-schnell")]
    FluxSchnell,
    #[serde(rename = "nano-banana")]
    NanoBanana,
    #[serde(rename = "nano-banana-2")]
    NanoBanana2,
    #[serde(rename = "nano-banana-pro")]
    NanoBananaPro,
    #[serde(rename = "seedream")]
    Seedream,
    #[serde(rename = "seedream-v4")]
    SeedreamV4,
    #[serde(rename = "z-image-turbo")]
    ZImageTurbo,
}
impl std::fmt::Display for V1AiImageGeneratorCreateBodyModelEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiImageGeneratorCreateBodyModelEnum::Default => "default",
            V1AiImageGeneratorCreateBodyModelEnum::FluxSchnell => "flux-schnell",
            V1AiImageGeneratorCreateBodyModelEnum::NanoBanana => "nano-banana",
            V1AiImageGeneratorCreateBodyModelEnum::NanoBanana2 => "nano-banana-2",
            V1AiImageGeneratorCreateBodyModelEnum::NanoBananaPro => "nano-banana-pro",
            V1AiImageGeneratorCreateBodyModelEnum::Seedream => "seedream",
            V1AiImageGeneratorCreateBodyModelEnum::SeedreamV4 => "seedream-v4",
            V1AiImageGeneratorCreateBodyModelEnum::ZImageTurbo => "z-image-turbo",
        };
        write!(f, "{}", str_val)
    }
}
