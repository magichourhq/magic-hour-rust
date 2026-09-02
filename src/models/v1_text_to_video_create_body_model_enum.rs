/// The AI model to use for video generation.
///
/// * `default`: uses our currently recommended model for general use. For paid tiers, defaults to `kling-3.0`. For free tiers, it defaults to `ltx-2.3`.
/// * `gemini-omni-1.1`: Best for precise short clips, first/last frames, and high-resolution output.
/// * `kling-2.6`: Best for action, motion blur, and controlled camera moves.
/// * `kling-3.0`: Best for cinematic stories, references, and optional audio.
/// * `ltx-2.3`: Fastest for general scenes, long clips, audio, and rapid iteration.
/// * `ltx-2.5`: Fastest for general scenes, long clips, audio, and rapid iteration.
/// * `minimax-h3`: Great for reference-driven clips with native audio and longer durations.
/// * `seedance-1.5`: Best for smooth, consistent motion with an end frame.
/// * `seedance-2.0`: Best for reference-led clips with precise subject control.
/// * `seedance-2.0-mini`: Faster reference-led clips with consistent motion and audio.
/// * `seedance-2.5`: Best for premium realism, detail, and natural motion.
/// * `sora-2`: Best for creative concepts and longer clips with audio.
/// * `veo3.1`: Best for romantic interactions and expressive action, with realistic detail.
/// * `veo3.1-lite`: Balanced realism and audio at a lower cost than Veo 3.1.
/// * `wan-2.2`: Best for physical motion, action, and camera movement.
///
/// If you specify the deprecated model value that includes the `-audio` suffix, this will be the same as included `audio` as `true`.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1TextToVideoCreateBodyModelEnum {
    #[default]
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "gemini-omni-1.1")]
    GeminiOmni11,
    #[serde(rename = "kling-1.6")]
    Kling16,
    #[serde(rename = "kling-2.5")]
    Kling25,
    #[serde(rename = "kling-2.5-audio")]
    Kling25Audio,
    #[serde(rename = "kling-2.6")]
    Kling26,
    #[serde(rename = "kling-3.0")]
    Kling30,
    #[serde(rename = "ltx-2")]
    Ltx2,
    #[serde(rename = "ltx-2.3")]
    Ltx23,
    #[serde(rename = "ltx-2.5")]
    Ltx25,
    #[serde(rename = "minimax-h3")]
    MinimaxH3,
    #[serde(rename = "seedance")]
    Seedance,
    #[serde(rename = "seedance-1.5")]
    Seedance15,
    #[serde(rename = "seedance-2.0")]
    Seedance20,
    #[serde(rename = "seedance-2.0-mini")]
    Seedance20Mini,
    #[serde(rename = "seedance-2.5")]
    Seedance25,
    #[serde(rename = "sora-2")]
    Sora2,
    #[serde(rename = "veo3.1")]
    Veo31,
    #[serde(rename = "veo3.1-audio")]
    Veo31Audio,
    #[serde(rename = "veo3.1-lite")]
    Veo31Lite,
    #[serde(rename = "wan-2.2")]
    Wan22,
}
impl std::fmt::Display for V1TextToVideoCreateBodyModelEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1TextToVideoCreateBodyModelEnum::Default => "default",
            V1TextToVideoCreateBodyModelEnum::GeminiOmni11 => "gemini-omni-1.1",
            V1TextToVideoCreateBodyModelEnum::Kling16 => "kling-1.6",
            V1TextToVideoCreateBodyModelEnum::Kling25 => "kling-2.5",
            V1TextToVideoCreateBodyModelEnum::Kling25Audio => "kling-2.5-audio",
            V1TextToVideoCreateBodyModelEnum::Kling26 => "kling-2.6",
            V1TextToVideoCreateBodyModelEnum::Kling30 => "kling-3.0",
            V1TextToVideoCreateBodyModelEnum::Ltx2 => "ltx-2",
            V1TextToVideoCreateBodyModelEnum::Ltx23 => "ltx-2.3",
            V1TextToVideoCreateBodyModelEnum::Ltx25 => "ltx-2.5",
            V1TextToVideoCreateBodyModelEnum::MinimaxH3 => "minimax-h3",
            V1TextToVideoCreateBodyModelEnum::Seedance => "seedance",
            V1TextToVideoCreateBodyModelEnum::Seedance15 => "seedance-1.5",
            V1TextToVideoCreateBodyModelEnum::Seedance20 => "seedance-2.0",
            V1TextToVideoCreateBodyModelEnum::Seedance20Mini => "seedance-2.0-mini",
            V1TextToVideoCreateBodyModelEnum::Seedance25 => "seedance-2.5",
            V1TextToVideoCreateBodyModelEnum::Sora2 => "sora-2",
            V1TextToVideoCreateBodyModelEnum::Veo31 => "veo3.1",
            V1TextToVideoCreateBodyModelEnum::Veo31Audio => "veo3.1-audio",
            V1TextToVideoCreateBodyModelEnum::Veo31Lite => "veo3.1-lite",
            V1TextToVideoCreateBodyModelEnum::Wan22 => "wan-2.2",
        };
        write!(f, "{}", str_val)
    }
}
