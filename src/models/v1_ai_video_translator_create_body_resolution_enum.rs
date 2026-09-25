/// Output video resolution. Defaults to 480p. 720p and 1080p require a paid plan.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiVideoTranslatorCreateBodyResolutionEnum {
    #[default]
    #[serde(rename = "1080p")]
    Enum1080p,
    #[serde(rename = "480p")]
    Enum480p,
    #[serde(rename = "720p")]
    Enum720p,
}
impl std::fmt::Display for V1AiVideoTranslatorCreateBodyResolutionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiVideoTranslatorCreateBodyResolutionEnum::Enum1080p => "1080p",
            V1AiVideoTranslatorCreateBodyResolutionEnum::Enum480p => "480p",
            V1AiVideoTranslatorCreateBodyResolutionEnum::Enum720p => "720p",
        };
        write!(f, "{}", str_val)
    }
}
