/// The AI model to use for video generation.
///
/// * `default`: uses our currently recommended model for general use. For paid tiers, defaults to `kling-3.0`. For free tiers, it defaults to `ltx-2`.
/// * `ltx-2`: Fast iteration with audio and lip-sync
/// * `wan-2.2`: Fast, strong visuals with effects
/// * `seedance`: Fast iteration and start/end frames
/// * `seedance-2.0`: State-of-the-art quality and consistency
/// * `kling-2.5`: Motion, action, and camera control
/// * `kling-3.0`: Cinematic, multi-scene storytelling
/// * `sora-2`: Story-first concepts and creativity
/// * `veo3.1`: Realistic visuals and prompt adherence
/// * `veo3.1-lite`: Good for fast, affordable, high-quality daily generation.
///
/// Legacy models:
/// * `kling-1.6`: Reliable baseline with smooth motion
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
    #[serde(rename = "seedance")]
    Seedance,
    #[serde(rename = "seedance-2.0")]
    Seedance20,
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
            V1ImageToVideoCreateBodyModelEnum::Seedance => "seedance",
            V1ImageToVideoCreateBodyModelEnum::Seedance20 => "seedance-2.0",
            V1ImageToVideoCreateBodyModelEnum::Sora2 => "sora-2",
            V1ImageToVideoCreateBodyModelEnum::Veo31 => "veo3.1",
            V1ImageToVideoCreateBodyModelEnum::Veo31Audio => "veo3.1-audio",
            V1ImageToVideoCreateBodyModelEnum::Veo31Lite => "veo3.1-lite",
            V1ImageToVideoCreateBodyModelEnum::Wan22 => "wan-2.2",
        };
        write!(f, "{}", str_val)
    }
}
