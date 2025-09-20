/// Controls overall motion style.
/// * `pro` -  Higher fidelity, realistic detail, accurate lip sync, and faster generation.
/// * `standard` -  More expressive motion, but lower visual fidelity.
///
/// * `expressive` - More motion and facial expressiveness; may introduce visual artifacts. (Deprecated: passing this value will be treated as `standard`)
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
    #[serde(rename = "standard")]
    Standard,
}
impl std::fmt::Display for V1AiTalkingPhotoCreateBodyStyleGenerationModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiTalkingPhotoCreateBodyStyleGenerationModeEnum::Expressive => "expressive",
            V1AiTalkingPhotoCreateBodyStyleGenerationModeEnum::Pro => "pro",
            V1AiTalkingPhotoCreateBodyStyleGenerationModeEnum::Stable => "stable",
            V1AiTalkingPhotoCreateBodyStyleGenerationModeEnum::Standard => "standard",
        };
        write!(f, "{}", str_val)
    }
}
