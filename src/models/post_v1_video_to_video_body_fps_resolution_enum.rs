/// Determines whether the resulting video will have the same frame per second as the original video, or half.
/// * `FULL` - the result video will have the same FPS as the input video
/// * `HALF` - the result video will have half the FPS as the input video
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum PostV1VideoToVideoBodyFpsResolutionEnum {
    #[default]
    #[serde(rename = "FULL")]
    Full,
    #[serde(rename = "HALF")]
    Half,
}
impl std::fmt::Display for PostV1VideoToVideoBodyFpsResolutionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostV1VideoToVideoBodyFpsResolutionEnum::Full => "FULL",
            PostV1VideoToVideoBodyFpsResolutionEnum::Half => "HALF",
        };
        write!(f, "{}", str_val)
    }
}
