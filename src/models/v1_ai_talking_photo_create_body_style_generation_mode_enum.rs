/// Controls overall motion style.
/// * `realistic` - Maintains likeness well, high quality, and reliable.
/// * `prompted` - Slightly lower likeness; allows option to prompt scene.
///
/// **Deprecated values (maintained for backward compatibility):**
/// * `pro` - Deprecated: use `realistic`
/// * `standard` - Deprecated: use `prompted`
/// * `stable` - Deprecated: use `realistic`
/// * `expressive` - Deprecated: use `prompted`
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiTalkingPhotoCreateBodyStyleGenerationModeEnum {
    #[default]
    #[serde(rename = "expressive")]
    Expressive,
    #[serde(rename = "pro")]
    Pro,
    #[serde(rename = "prompted")]
    Prompted,
    #[serde(rename = "realistic")]
    Realistic,
    #[serde(rename = "stable")]
    Stable,
    #[serde(rename = "standard")]
    Standard,
}
impl std::fmt::Display for V1AiTalkingPhotoCreateBodyStyleGenerationModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiTalkingPhotoCreateBodyStyleGenerationModeEnum::Expressive => "expressive",
            V1AiTalkingPhotoCreateBodyStyleGenerationModeEnum::Pro => "pro",
            V1AiTalkingPhotoCreateBodyStyleGenerationModeEnum::Prompted => "prompted",
            V1AiTalkingPhotoCreateBodyStyleGenerationModeEnum::Realistic => "realistic",
            V1AiTalkingPhotoCreateBodyStyleGenerationModeEnum::Stable => "stable",
            V1AiTalkingPhotoCreateBodyStyleGenerationModeEnum::Standard => "standard",
        };
        write!(f, "{}", str_val)
    }
}
