/// Output resolution. `720p` and `1080p` set the output's short side, `2k` and `4k` its long side; the aspect ratio is kept. Requires a paid plan.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiVideoUpscalerCreateBodyResolutionEnum {
    #[default]
    #[serde(rename = "1080p")]
    Enum1080p,
    #[serde(rename = "2k")]
    Enum2k,
    #[serde(rename = "4k")]
    Enum4k,
    #[serde(rename = "720p")]
    Enum720p,
}
impl std::fmt::Display for V1AiVideoUpscalerCreateBodyResolutionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiVideoUpscalerCreateBodyResolutionEnum::Enum1080p => "1080p",
            V1AiVideoUpscalerCreateBodyResolutionEnum::Enum2k => "2k",
            V1AiVideoUpscalerCreateBodyResolutionEnum::Enum4k => "4k",
            V1AiVideoUpscalerCreateBodyResolutionEnum::Enum720p => "720p",
        };
        write!(f, "{}", str_val)
    }
}
