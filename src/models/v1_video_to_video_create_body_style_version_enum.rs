/// * `v1` - more detail, closer prompt adherence, and frame-by-frame previews.
/// * `v2` - faster, more consistent, and less noisy.
/// * `v3` - consistent styling guided by a caller-supplied reference image. Requires assets.initial_image_path and positive width/height.
/// * `default` - use the selected art style's default version, including v3.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1VideoToVideoCreateBodyStyleVersionEnum {
    #[default]
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "v1")]
    V1,
    #[serde(rename = "v2")]
    V2,
    #[serde(rename = "v3")]
    V3,
}
impl std::fmt::Display for V1VideoToVideoCreateBodyStyleVersionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1VideoToVideoCreateBodyStyleVersionEnum::Default => "default",
            V1VideoToVideoCreateBodyStyleVersionEnum::V1 => "v1",
            V1VideoToVideoCreateBodyStyleVersionEnum::V2 => "v2",
            V1VideoToVideoCreateBodyStyleVersionEnum::V3 => "v3",
        };
        write!(f, "{}", str_val)
    }
}
