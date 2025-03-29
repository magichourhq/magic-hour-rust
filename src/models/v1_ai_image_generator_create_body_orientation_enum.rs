/// V1AiImageGeneratorCreateBodyOrientationEnum
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiImageGeneratorCreateBodyOrientationEnum {
    #[default]
    #[serde(rename = "landscape")]
    Landscape,
    #[serde(rename = "portrait")]
    Portrait,
    #[serde(rename = "square")]
    Square,
}
impl std::fmt::Display for V1AiImageGeneratorCreateBodyOrientationEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiImageGeneratorCreateBodyOrientationEnum::Landscape => "landscape",
            V1AiImageGeneratorCreateBodyOrientationEnum::Portrait => "portrait",
            V1AiImageGeneratorCreateBodyOrientationEnum::Square => "square",
        };
        write!(f, "{}", str_val)
    }
}
