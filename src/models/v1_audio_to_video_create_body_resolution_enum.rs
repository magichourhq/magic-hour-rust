/// Output video resolution. Defaults to `720p` on paid tiers and `480p` on free tiers.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AudioToVideoCreateBodyResolutionEnum {
    #[default]
    #[serde(rename = "1080p")]
    Enum1080p,
    #[serde(rename = "480p")]
    Enum480p,
    #[serde(rename = "720p")]
    Enum720p,
}
impl std::fmt::Display for V1AudioToVideoCreateBodyResolutionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AudioToVideoCreateBodyResolutionEnum::Enum1080p => "1080p",
            V1AudioToVideoCreateBodyResolutionEnum::Enum480p => "480p",
            V1AudioToVideoCreateBodyResolutionEnum::Enum720p => "720p",
        };
        write!(f, "{}", str_val)
    }
}
