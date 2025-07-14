/// DEPRECATED: Please use `resolution` field instead. For backward compatibility:
/// * `quick` maps to 720p resolution
/// * `studio` maps to 1080p resolution
///
/// This field will be removed in a future version. Use the `resolution` field to directly to specify the resolution.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1ImageToVideoCreateBodyStyleQualityModeEnum {
    #[default]
    #[serde(rename = "quick")]
    Quick,
    #[serde(rename = "studio")]
    Studio,
}
impl std::fmt::Display for V1ImageToVideoCreateBodyStyleQualityModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1ImageToVideoCreateBodyStyleQualityModeEnum::Quick => "quick",
            V1ImageToVideoCreateBodyStyleQualityModeEnum::Studio => "studio",
        };
        write!(f, "{}", str_val)
    }
}
