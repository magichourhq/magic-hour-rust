/// Deprecated. Use `aspect_ratio` instead.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1TextToVideoCreateBodyOrientationEnum {
    #[default]
    #[serde(rename = "landscape")]
    Landscape,
    #[serde(rename = "portrait")]
    Portrait,
    #[serde(rename = "square")]
    Square,
}
impl std::fmt::Display for V1TextToVideoCreateBodyOrientationEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1TextToVideoCreateBodyOrientationEnum::Landscape => "landscape",
            V1TextToVideoCreateBodyOrientationEnum::Portrait => "portrait",
            V1TextToVideoCreateBodyOrientationEnum::Square => "square",
        };
        write!(f, "{}", str_val)
    }
}
