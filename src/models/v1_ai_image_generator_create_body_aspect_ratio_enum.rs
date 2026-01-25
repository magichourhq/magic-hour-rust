/// The aspect ratio of the output image(s). If not specified, defaults to `1:1` (square).
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiImageGeneratorCreateBodyAspectRatioEnum {
    #[default]
    #[serde(rename = "16:9")]
    Enum169,
    #[serde(rename = "1:1")]
    Enum11,
    #[serde(rename = "9:16")]
    Enum916,
}
impl std::fmt::Display for V1AiImageGeneratorCreateBodyAspectRatioEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiImageGeneratorCreateBodyAspectRatioEnum::Enum169 => "16:9",
            V1AiImageGeneratorCreateBodyAspectRatioEnum::Enum11 => "1:1",
            V1AiImageGeneratorCreateBodyAspectRatioEnum::Enum916 => "9:16",
        };
        write!(f, "{}", str_val)
    }
}
