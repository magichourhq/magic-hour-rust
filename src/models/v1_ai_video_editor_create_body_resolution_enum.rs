/// Output resolution. Defaults to `480p` for free tier and `720p` for paid. Supported per model: `gemini-omni-1.2`: 360p, 720p, 1080p, 4k; `wan-3.0`: 480p, 720p, 1080p; `ltx-2.5`: 480p, 720p, 1080p.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiVideoEditorCreateBodyResolutionEnum {
    #[default]
    #[serde(rename = "1080p")]
    Enum1080p,
    #[serde(rename = "360p")]
    Enum360p,
    #[serde(rename = "480p")]
    Enum480p,
    #[serde(rename = "4k")]
    Enum4k,
    #[serde(rename = "720p")]
    Enum720p,
}
impl std::fmt::Display for V1AiVideoEditorCreateBodyResolutionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiVideoEditorCreateBodyResolutionEnum::Enum1080p => "1080p",
            V1AiVideoEditorCreateBodyResolutionEnum::Enum360p => "360p",
            V1AiVideoEditorCreateBodyResolutionEnum::Enum480p => "480p",
            V1AiVideoEditorCreateBodyResolutionEnum::Enum4k => "4k",
            V1AiVideoEditorCreateBodyResolutionEnum::Enum720p => "720p",
        };
        write!(f, "{}", str_val)
    }
}
