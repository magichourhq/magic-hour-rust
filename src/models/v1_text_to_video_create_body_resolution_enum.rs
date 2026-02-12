/// Controls the output video resolution. Defaults to `720p` if not specified.
///
/// * **Default**: Supports `480p`, `720p`, and `1080p`.
/// * **seedance**: Supports `480p`, `720p`, `1080p`.
/// * **kling-2.5**: Supports `720p`, `1080p`.
/// * **kling-3.0**: Supports `720p`, `1080p`.
/// * **sora-2**: Supports `720p`.
/// * **veo3.1**: Supports `720p`, `1080p`.
/// * **kling-1.6**: Supports `720p`, `1080p`.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1TextToVideoCreateBodyResolutionEnum {
    #[default]
    #[serde(rename = "1080p")]
    Enum1080p,
    #[serde(rename = "480p")]
    Enum480p,
    #[serde(rename = "720p")]
    Enum720p,
}
impl std::fmt::Display for V1TextToVideoCreateBodyResolutionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1TextToVideoCreateBodyResolutionEnum::Enum1080p => "1080p",
            V1TextToVideoCreateBodyResolutionEnum::Enum480p => "480p",
            V1TextToVideoCreateBodyResolutionEnum::Enum720p => "720p",
        };
        write!(f, "{}", str_val)
    }
}
