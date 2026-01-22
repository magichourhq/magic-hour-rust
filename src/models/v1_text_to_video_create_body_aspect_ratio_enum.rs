/// Determines the aspect ratio of the output video.
/// * **Seedance**: Supports `9:16`, `16:9`, `1:1`.
/// * **Kling 2.5 Audio**: Supports `9:16`, `16:9`, `1:1`.
/// * **Sora 2**: Supports `9:16`, `16:9`.
/// * **Veo 3.1 Audio**: Supports `9:16`, `16:9`.
/// * **Veo 3.1**: Supports `9:16`, `16:9`.
/// * **Kling 1.6**: Supports `9:16`, `16:9`, `1:1`.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1TextToVideoCreateBodyAspectRatioEnum {
    #[default]
    #[serde(rename = "16:9")]
    Enum169,
    #[serde(rename = "1:1")]
    Enum11,
    #[serde(rename = "9:16")]
    Enum916,
}
impl std::fmt::Display for V1TextToVideoCreateBodyAspectRatioEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1TextToVideoCreateBodyAspectRatioEnum::Enum169 => "16:9",
            V1TextToVideoCreateBodyAspectRatioEnum::Enum11 => "1:1",
            V1TextToVideoCreateBodyAspectRatioEnum::Enum916 => "9:16",
        };
        write!(f, "{}", str_val)
    }
}
