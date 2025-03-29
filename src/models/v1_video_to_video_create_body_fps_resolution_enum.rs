/// Determines whether the resulting video will have the same frame per second as the original video, or half.
/// * `FULL` - the result video will have the same FPS as the input video
/// * `HALF` - the result video will have half the FPS as the input video
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1VideoToVideoCreateBodyFpsResolutionEnum {
    #[default]
    #[serde(rename = "FULL")]
    Full,
    #[serde(rename = "HALF")]
    Half,
}
impl std::fmt::Display for V1VideoToVideoCreateBodyFpsResolutionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1VideoToVideoCreateBodyFpsResolutionEnum::Full => "FULL",
            V1VideoToVideoCreateBodyFpsResolutionEnum::Half => "HALF",
        };
        write!(f, "{}", str_val)
    }
}
