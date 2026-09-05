/// Controls the output video resolution. Defaults to `720p` on paid tiers and `480p` on free tiers.
///
/// * **`gemini-omni-1.1`**: Supports 360p, 720p, 1080p, 4k.
/// * **`kling-2.6`**: Supports 720p, 1080p.
/// * **`kling-3.0`**: Supports 720p, 1080p, 4k.
/// * **`ltx-2.3`**: Supports 480p, 720p, 1080p.
/// * **`ltx-2.5`**: Supports 480p, 720p, 1080p.
/// * **`minimax-h3`**: Supports 480p, 720p, 1080p.
/// * **`seedance-1.5`**: Supports 480p, 720p, 1080p.
/// * **`seedance-2.0`**: Supports 480p, 720p.
/// * **`seedance-2.0-mini`**: Supports 480p, 720p.
/// * **`seedance-2.5`**: Supports 480p, 720p.
/// * **`sora-2`**: Supports 720p.
/// * **`veo3.1`**: Supports 720p, 1080p.
/// * **`veo3.1-lite`**: Supports 720p, 1080p.
/// * **`wan-2.2`**: Supports 480p, 720p, 1080p.
///
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1ImageToVideoCreateBodyResolutionEnum {
    #[default]
    #[serde(rename = "1080p")]
    Enum1080p,
    #[serde(rename = "360p")]
    Enum360p,
    #[serde(rename = "480p")]
    Enum480p,
    #[serde(rename = "4k")]
    Enum4k,
    #[serde(rename = "720p")]
    Enum720p,
}
impl std::fmt::Display for V1ImageToVideoCreateBodyResolutionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1ImageToVideoCreateBodyResolutionEnum::Enum1080p => "1080p",
            V1ImageToVideoCreateBodyResolutionEnum::Enum360p => "360p",
            V1ImageToVideoCreateBodyResolutionEnum::Enum480p => "480p",
            V1ImageToVideoCreateBodyResolutionEnum::Enum4k => "4k",
            V1ImageToVideoCreateBodyResolutionEnum::Enum720p => "720p",
        };
        write!(f, "{}", str_val)
    }
}
