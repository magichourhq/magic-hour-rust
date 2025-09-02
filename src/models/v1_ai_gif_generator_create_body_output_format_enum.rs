/// The output file format for the generated animation.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiGifGeneratorCreateBodyOutputFormatEnum {
    #[default]
    #[serde(rename = "gif")]
    Gif,
    #[serde(rename = "mp4")]
    Mp4,
    #[serde(rename = "webm")]
    Webm,
}
impl std::fmt::Display for V1AiGifGeneratorCreateBodyOutputFormatEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiGifGeneratorCreateBodyOutputFormatEnum::Gif => "gif",
            V1AiGifGeneratorCreateBodyOutputFormatEnum::Mp4 => "mp4",
            V1AiGifGeneratorCreateBodyOutputFormatEnum::Webm => "webm",
        };
        write!(f, "{}", str_val)
    }
}
