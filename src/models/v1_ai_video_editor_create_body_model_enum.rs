/// The AI model to use for video editing. Defaults to `ltx-2.5` on free tiers and `gemini-omni-1.2` on paid tiers.
///
/// * `gemini-omni-1.2`: Best for precise clips up to 15s, references, and high-resolution output. Requires a paid tier.
/// * `wan-3.0`: High-quality video with native audio, long clips, and end-frame control. Requires a paid tier.
/// * `ltx-2.5`: Fastest for general scenes, long clips, audio, and rapid iteration.
/// * `kling-4.0`: Create cinematic videos with image and video references and optional audio. Requires a paid tier.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiVideoEditorCreateBodyModelEnum {
    #[default]
    #[serde(rename = "gemini-omni")]
    GeminiOmni,
    #[serde(rename = "gemini-omni-1.1")]
    GeminiOmni11,
    #[serde(rename = "gemini-omni-1.2")]
    GeminiOmni12,
    #[serde(rename = "kling-4.0")]
    Kling40,
    #[serde(rename = "ltx-2.3")]
    Ltx23,
    #[serde(rename = "ltx-2.5")]
    Ltx25,
    #[serde(rename = "wan-3.0")]
    Wan30,
}
impl std::fmt::Display for V1AiVideoEditorCreateBodyModelEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiVideoEditorCreateBodyModelEnum::GeminiOmni => "gemini-omni",
            V1AiVideoEditorCreateBodyModelEnum::GeminiOmni11 => "gemini-omni-1.1",
            V1AiVideoEditorCreateBodyModelEnum::GeminiOmni12 => "gemini-omni-1.2",
            V1AiVideoEditorCreateBodyModelEnum::Kling40 => "kling-4.0",
            V1AiVideoEditorCreateBodyModelEnum::Ltx23 => "ltx-2.3",
            V1AiVideoEditorCreateBodyModelEnum::Ltx25 => "ltx-2.5",
            V1AiVideoEditorCreateBodyModelEnum::Wan30 => "wan-3.0",
        };
        write!(f, "{}", str_val)
    }
}
