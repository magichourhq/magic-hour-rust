/// DEPRECATED: Use `model` field instead for explicit model selection.
///
/// Legacy quality mode mapping:
/// - `standard` → `z-image-turbo` model
/// - `pro` → `seedream` model
///
/// If model is specified, it will take precedence over the legacy quality_mode field.
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
