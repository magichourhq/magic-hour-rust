/// * `v1` - more detail, closer prompt adherence, and frame-by-frame previews.
/// * `v2` - faster, more consistent, and less noisy.
/// * `default` - use the selected art style's default version. Some art styles default to v3, which is unavailable through this API. For `Studio Ghibli`, explicitly set `version` to `v2` to use its supported preset.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1VideoToVideoCreateBodyStyleVersionEnum {
    #[default]
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "v1")]
    V1,
    #[serde(rename = "v2")]
    V2,
}
impl std::fmt::Display for V1VideoToVideoCreateBodyStyleVersionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1VideoToVideoCreateBodyStyleVersionEnum::Default => "default",
            V1VideoToVideoCreateBodyStyleVersionEnum::V1 => "v1",
            V1VideoToVideoCreateBodyStyleVersionEnum::V2 => "v2",
        };
        write!(f, "{}", str_val)
    }
}
