/// Defaults to 720p on paid plans and 480p on free plans. Unsupported model/resolution combinations and resolutions unavailable for your plan are rejected.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiUgcAdCreateBodyResolutionEnum {
    #[default]
    #[serde(rename = "1080p")]
    Enum1080p,
    #[serde(rename = "480p")]
    Enum480p,
    #[serde(rename = "720p")]
    Enum720p,
}
impl std::fmt::Display for V1AiUgcAdCreateBodyResolutionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiUgcAdCreateBodyResolutionEnum::Enum1080p => "1080p",
            V1AiUgcAdCreateBodyResolutionEnum::Enum480p => "480p",
            V1AiUgcAdCreateBodyResolutionEnum::Enum720p => "720p",
        };
        write!(f, "{}", str_val)
    }
}
