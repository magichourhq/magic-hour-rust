/// Output preset. Overrides `max_resolution`. Defaults to 480p in prompted mode; realistic mode uses the mapped plan maximum.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiTalkingPhotoCreateBodyResolutionEnum {
    #[default]
    #[serde(rename = "480p")]
    Enum480p,
    #[serde(rename = "720p")]
    Enum720p,
}
impl std::fmt::Display for V1AiTalkingPhotoCreateBodyResolutionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiTalkingPhotoCreateBodyResolutionEnum::Enum480p => "480p",
            V1AiTalkingPhotoCreateBodyResolutionEnum::Enum720p => "720p",
        };
        write!(f, "{}", str_val)
    }
}
