/// * `quick` - Fastest option for rapid results. Takes ~3 minutes per 5s of video.
/// *  `studio` - Polished visuals with longer runtime. Takes ~8.5 minutes per 5s of video.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1TextToVideoCreateBodyStyleQualityModeEnum {
    #[default]
    #[serde(rename = "quick")]
    Quick,
    #[serde(rename = "studio")]
    Studio,
}
impl std::fmt::Display for V1TextToVideoCreateBodyStyleQualityModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1TextToVideoCreateBodyStyleQualityModeEnum::Quick => "quick",
            V1TextToVideoCreateBodyStyleQualityModeEnum::Studio => "studio",
        };
        write!(f, "{}", str_val)
    }
}
