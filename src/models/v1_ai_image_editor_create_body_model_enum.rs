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
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiImageEditorCreateBodyModelEnum {
    #[default]
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "nano-banana")]
    NanoBanana,
    #[serde(rename = "nano-banana-pro")]
    NanoBananaPro,
    #[serde(rename = "qwen-edit")]
    QwenEdit,
    #[serde(rename = "seedream-v4")]
    SeedreamV4,
    #[serde(rename = "seedream-v4.5")]
    SeedreamV45,
}
impl std::fmt::Display for V1AiImageEditorCreateBodyModelEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiImageEditorCreateBodyModelEnum::Default => "default",
            V1AiImageEditorCreateBodyModelEnum::NanoBanana => "nano-banana",
            V1AiImageEditorCreateBodyModelEnum::NanoBananaPro => "nano-banana-pro",
            V1AiImageEditorCreateBodyModelEnum::QwenEdit => "qwen-edit",
            V1AiImageEditorCreateBodyModelEnum::SeedreamV4 => "seedream-v4",
            V1AiImageEditorCreateBodyModelEnum::SeedreamV45 => "seedream-v4.5",
        };
        write!(f, "{}", str_val)
    }
}
