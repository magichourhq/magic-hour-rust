/// Maximum resolution for the generated image.
///
/// **Options:**
/// - `auto` - Automatic resolution (all tiers, default)
/// - `2k` - Up to 2048px (requires Pro or Business tier)
/// - `4k` - Up to 4096px (requires Business tier)
///
/// Note: Resolution availability depends on the model and your subscription tier. See `model` field for which resolutions each model supports. Defaults to `auto` if not specified.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiImageGeneratorCreateBodyResolutionEnum {
    #[default]
    #[serde(rename = "2k")]
    Enum2k,
    #[serde(rename = "4k")]
    Enum4k,
    #[serde(rename = "auto")]
    Auto,
}
impl std::fmt::Display for V1AiImageGeneratorCreateBodyResolutionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiImageGeneratorCreateBodyResolutionEnum::Enum2k => "2k",
            V1AiImageGeneratorCreateBodyResolutionEnum::Enum4k => "4k",
            V1AiImageGeneratorCreateBodyResolutionEnum::Auto => "auto",
        };
        write!(f, "{}", str_val)
    }
}
