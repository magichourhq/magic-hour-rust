/// The upscaling mode. `"pro"` is faster and does not require `enhancement`. `"creative"` requires `enhancement`. Defaults to `"creative"`.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiImageUpscalerCreateBodyStyleModeEnum {
    #[default]
    #[serde(rename = "creative")]
    Creative,
    #[serde(rename = "pro")]
    Pro,
}
impl std::fmt::Display for V1AiImageUpscalerCreateBodyStyleModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiImageUpscalerCreateBodyStyleModeEnum::Creative => "creative",
            V1AiImageUpscalerCreateBodyStyleModeEnum::Pro => "pro",
        };
        write!(f, "{}", str_val)
    }
}
