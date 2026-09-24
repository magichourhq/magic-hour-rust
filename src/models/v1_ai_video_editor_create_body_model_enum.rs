/// Editing model. Defaults to LTX 2.5 for free tier and `gemini-omni-1.1` for paid. `gemini-omni` is deprecated; use `gemini-omni-1.1` instead.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiVideoEditorCreateBodyModelEnum {
    #[default]
    #[serde(rename = "gemini-omni")]
    GeminiOmni,
    #[serde(rename = "gemini-omni-1.1")]
    GeminiOmni11,
    #[serde(rename = "ltx-2.3")]
    Ltx23,
    #[serde(rename = "ltx-2.5")]
    Ltx25,
}
impl std::fmt::Display for V1AiVideoEditorCreateBodyModelEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiVideoEditorCreateBodyModelEnum::GeminiOmni => "gemini-omni",
            V1AiVideoEditorCreateBodyModelEnum::GeminiOmni11 => "gemini-omni-1.1",
            V1AiVideoEditorCreateBodyModelEnum::Ltx23 => "ltx-2.3",
            V1AiVideoEditorCreateBodyModelEnum::Ltx25 => "ltx-2.5",
        };
        write!(f, "{}", str_val)
    }
}
