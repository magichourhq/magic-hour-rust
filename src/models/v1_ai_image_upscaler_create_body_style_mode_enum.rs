/// The upscaling mode. `"preserve"` uses the fast pro pipeline (1× credit multiplier). `"balanced"` and `"creative"` use the creative pipeline (2× credit multiplier). `"pro"` is deprecated and maps to `"preserve"`. Defaults to `"balanced"`.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiImageUpscalerCreateBodyStyleModeEnum {
    #[default]
    #[serde(rename = "balanced")]
    Balanced,
    #[serde(rename = "creative")]
    Creative,
    #[serde(rename = "preserve")]
    Preserve,
    #[serde(rename = "pro")]
    Pro,
}
impl std::fmt::Display for V1AiImageUpscalerCreateBodyStyleModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiImageUpscalerCreateBodyStyleModeEnum::Balanced => "balanced",
            V1AiImageUpscalerCreateBodyStyleModeEnum::Creative => "creative",
            V1AiImageUpscalerCreateBodyStyleModeEnum::Preserve => "preserve",
            V1AiImageUpscalerCreateBodyStyleModeEnum::Pro => "pro",
        };
        write!(f, "{}", str_val)
    }
}
