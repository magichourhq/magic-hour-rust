/// Preset subtitle templates. Please visit https://magichour.ai/create/auto-subtitle-generator to see the style of the existing templates.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AutoSubtitleGeneratorCreateBodyStyleTemplateEnum {
    #[default]
    #[serde(rename = "cinematic")]
    Cinematic,
    #[serde(rename = "highlight")]
    Highlight,
    #[serde(rename = "karaoke")]
    Karaoke,
    #[serde(rename = "minimalist")]
    Minimalist,
}
impl std::fmt::Display for V1AutoSubtitleGeneratorCreateBodyStyleTemplateEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AutoSubtitleGeneratorCreateBodyStyleTemplateEnum::Cinematic => "cinematic",
            V1AutoSubtitleGeneratorCreateBodyStyleTemplateEnum::Highlight => "highlight",
            V1AutoSubtitleGeneratorCreateBodyStyleTemplateEnum::Karaoke => "karaoke",
            V1AutoSubtitleGeneratorCreateBodyStyleTemplateEnum::Minimalist => {
                "minimalist"
            }
        };
        write!(f, "{}", str_val)
    }
}
