/// Editing model. Defaults to `ltx-2.3` for free tier and `gemini-omni` for paid. Use `ltx-2.3` for LTX video edit.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiVideoEditorCreateBodyModelEnum {
    #[default]
    #[serde(rename = "gemini-omni")]
    GeminiOmni,
    #[serde(rename = "ltx-2.3")]
    Ltx23,
}
impl std::fmt::Display for V1AiVideoEditorCreateBodyModelEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiVideoEditorCreateBodyModelEnum::GeminiOmni => "gemini-omni",
            V1AiVideoEditorCreateBodyModelEnum::Ltx23 => "ltx-2.3",
        };
        write!(f, "{}", str_val)
    }
}
