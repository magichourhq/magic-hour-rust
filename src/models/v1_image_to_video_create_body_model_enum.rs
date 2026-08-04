/// The AI model to use for video generation.
///
/// * `default`: uses our currently recommended model for general use. For paid tiers, defaults to `kling-3.0`. For free tiers, it defaults to `ltx-2.3`.
/// * `ltx-2.3`: Fastest output. Best for rapid iteration.
/// * `wan-2.2`: Strong physics, camera moves, and motion.
/// * `kling-3.0`: Best overall quality for cinematic storytelling.
/// * `veo3.1-lite`: Veo quality at a more accessible cost.
/// * `veo3.1`: Google's model. Highest realism and detail.
/// * `seedance-1.5`: Smooth, consistent motion with precision.
/// * `seedance-2.0-mini`: Fast, consistent video with strong motion quality
/// * `seedance-2.0`: Top quality with reference-to-video control.
/// * `sora-2`: Open AI's model. Great for creativity and viral clips.
///
/// If you specify the deprecated model value that includes the `-audio` suffix, this will be the same as included `audio` as `true`.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1ImageToVideoCreateBodyModelEnum {
    #[default]
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "kling-1.6")]
    Kling16,
    #[serde(rename = "kling-2.5")]
    Kling25,
    #[serde(rename = "kling-2.5-audio")]
    Kling25Audio,
    #[serde(rename = "kling-3.0")]
    Kling30,
    #[serde(rename = "ltx-2")]
    Ltx2,
    #[serde(rename = "ltx-2.3")]
    Ltx23,
    #[serde(rename = "seedance")]
    Seedance,
    #[serde(rename = "seedance-1.5")]
    Seedance15,
    #[serde(rename = "seedance-2.0")]
    Seedance20,
    #[serde(rename = "seedance-2.0-mini")]
    Seedance20Mini,
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
impl std::fmt::Display for V1ImageToVideoCreateBodyModelEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1ImageToVideoCreateBodyModelEnum::Default => "default",
            V1ImageToVideoCreateBodyModelEnum::Kling16 => "kling-1.6",
            V1ImageToVideoCreateBodyModelEnum::Kling25 => "kling-2.5",
            V1ImageToVideoCreateBodyModelEnum::Kling25Audio => "kling-2.5-audio",
            V1ImageToVideoCreateBodyModelEnum::Kling30 => "kling-3.0",
            V1ImageToVideoCreateBodyModelEnum::Ltx2 => "ltx-2",
            V1ImageToVideoCreateBodyModelEnum::Ltx23 => "ltx-2.3",
            V1ImageToVideoCreateBodyModelEnum::Seedance => "seedance",
            V1ImageToVideoCreateBodyModelEnum::Seedance15 => "seedance-1.5",
            V1ImageToVideoCreateBodyModelEnum::Seedance20 => "seedance-2.0",
            V1ImageToVideoCreateBodyModelEnum::Seedance20Mini => "seedance-2.0-mini",
            V1ImageToVideoCreateBodyModelEnum::Sora2 => "sora-2",
            V1ImageToVideoCreateBodyModelEnum::Veo31 => "veo3.1",
            V1ImageToVideoCreateBodyModelEnum::Veo31Audio => "veo3.1-audio",
            V1ImageToVideoCreateBodyModelEnum::Veo31Lite => "veo3.1-lite",
            V1ImageToVideoCreateBodyModelEnum::Wan22 => "wan-2.2",
        };
        write!(f, "{}", str_val)
    }
}
