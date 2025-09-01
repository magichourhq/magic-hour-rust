/// Controls overall motion style.
/// * `pro` -  Realistic, high fidelity, accurate lip sync, slower.
/// * `expressive` - More motion and facial expressiveness; may introduce visual artifacts.
/// * `stable` -  Reduced motion for cleaner output; may result in minimal animation. (Deprecated: passing this value will be treated as `pro`)
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiTalkingPhotoCreateBodyStyleGenerationModeEnum {
    #[default]
    #[serde(rename = "expressive")]
    Expressive,
    #[serde(rename = "pro")]
    Pro,
    #[serde(rename = "stable")]
    Stable,
}
impl std::fmt::Display for V1AiTalkingPhotoCreateBodyStyleGenerationModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiTalkingPhotoCreateBodyStyleGenerationModeEnum::Expressive => "expressive",
            V1AiTalkingPhotoCreateBodyStyleGenerationModeEnum::Pro => "pro",
            V1AiTalkingPhotoCreateBodyStyleGenerationModeEnum::Stable => "stable",
        };
        write!(f, "{}", str_val)
    }
}
