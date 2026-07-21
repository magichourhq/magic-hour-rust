/// Output resolution. Defaults to `480p` for free tier and `720p` for paid. Google Omni supports 720p only; LTX-2.3 supports 480p, 720p, and 1080p.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiVideoEditorCreateBodyResolutionEnum {
    #[default]
    #[serde(rename = "1080p")]
    Enum1080p,
    #[serde(rename = "480p")]
    Enum480p,
    #[serde(rename = "720p")]
    Enum720p,
}
impl std::fmt::Display for V1AiVideoEditorCreateBodyResolutionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiVideoEditorCreateBodyResolutionEnum::Enum1080p => "1080p",
            V1AiVideoEditorCreateBodyResolutionEnum::Enum480p => "480p",
            V1AiVideoEditorCreateBodyResolutionEnum::Enum720p => "720p",
        };
        write!(f, "{}", str_val)
    }
}
