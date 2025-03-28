/// * `v1` - more detail, closer prompt adherence, and frame-by-frame previews.
/// * `v2` - faster, more consistent, and less noisy.
/// * `default` - use the default version for the selected art style.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum PostV1VideoToVideoBodyStyleVersionEnum {
    #[default]
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "v1")]
    V1,
    #[serde(rename = "v2")]
    V2,
}
impl std::fmt::Display for PostV1VideoToVideoBodyStyleVersionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostV1VideoToVideoBodyStyleVersionEnum::Default => "default",
            PostV1VideoToVideoBodyStyleVersionEnum::V1 => "v1",
            PostV1VideoToVideoBodyStyleVersionEnum::V2 => "v2",
        };
        write!(f, "{}", str_val)
    }
}
