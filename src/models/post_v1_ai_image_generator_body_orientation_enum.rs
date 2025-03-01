/// PostV1AiImageGeneratorBodyOrientationEnum
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum PostV1AiImageGeneratorBodyOrientationEnum {
    #[default]
    #[serde(rename = "landscape")]
    Landscape,
    #[serde(rename = "portrait")]
    Portrait,
    #[serde(rename = "square")]
    Square,
}
impl std::fmt::Display for PostV1AiImageGeneratorBodyOrientationEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostV1AiImageGeneratorBodyOrientationEnum::Landscape => "landscape",
            PostV1AiImageGeneratorBodyOrientationEnum::Portrait => "portrait",
            PostV1AiImageGeneratorBodyOrientationEnum::Square => "square",
        };
        write!(f, "{}", str_val)
    }
}
