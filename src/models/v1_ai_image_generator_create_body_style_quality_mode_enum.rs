/// Controls the quality of the generated image. Defaults to 'standard' if not specified.
///
/// **Options:**
/// - `standard` - Standard quality generation. Cost: 5 credits per image.
/// - `pro` - Pro quality generation with enhanced details and quality. Cost: 30 credits per image.
///
/// Note: Pro mode is available for users on Creator, Pro, or Business tier.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiImageGeneratorCreateBodyStyleQualityModeEnum {
    #[default]
    #[serde(rename = "pro")]
    Pro,
    #[serde(rename = "standard")]
    Standard,
}
impl std::fmt::Display for V1AiImageGeneratorCreateBodyStyleQualityModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiImageGeneratorCreateBodyStyleQualityModeEnum::Pro => "pro",
            V1AiImageGeneratorCreateBodyStyleQualityModeEnum::Standard => "standard",
        };
        write!(f, "{}", str_val)
    }
}
