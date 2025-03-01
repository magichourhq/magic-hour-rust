/// Determines the orientation of the output video
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum PostV1TextToVideoBodyOrientationEnum {
    #[default]
    #[serde(rename = "landscape")]
    Landscape,
    #[serde(rename = "portrait")]
    Portrait,
    #[serde(rename = "square")]
    Square,
}
impl std::fmt::Display for PostV1TextToVideoBodyOrientationEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostV1TextToVideoBodyOrientationEnum::Landscape => "landscape",
            PostV1TextToVideoBodyOrientationEnum::Portrait => "portrait",
            PostV1TextToVideoBodyOrientationEnum::Square => "square",
        };
        write!(f, "{}", str_val)
    }
}
