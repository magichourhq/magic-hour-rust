/// The aspect ratio of the output image(s). If not specified, defaults to `auto`.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiImageEditorCreateBodyAspectRatioEnum {
    #[default]
    #[serde(rename = "16:9")]
    Enum169,
    #[serde(rename = "1:1")]
    Enum11,
    #[serde(rename = "2:3")]
    Enum23,
    #[serde(rename = "3:2")]
    Enum32,
    #[serde(rename = "4:3")]
    Enum43,
    #[serde(rename = "4:5")]
    Enum45,
    #[serde(rename = "9:16")]
    Enum916,
    #[serde(rename = "auto")]
    Auto,
}
impl std::fmt::Display for V1AiImageEditorCreateBodyAspectRatioEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiImageEditorCreateBodyAspectRatioEnum::Enum169 => "16:9",
            V1AiImageEditorCreateBodyAspectRatioEnum::Enum11 => "1:1",
            V1AiImageEditorCreateBodyAspectRatioEnum::Enum23 => "2:3",
            V1AiImageEditorCreateBodyAspectRatioEnum::Enum32 => "3:2",
            V1AiImageEditorCreateBodyAspectRatioEnum::Enum43 => "4:3",
            V1AiImageEditorCreateBodyAspectRatioEnum::Enum45 => "4:5",
            V1AiImageEditorCreateBodyAspectRatioEnum::Enum916 => "9:16",
            V1AiImageEditorCreateBodyAspectRatioEnum::Auto => "auto",
        };
        write!(f, "{}", str_val)
    }
}
