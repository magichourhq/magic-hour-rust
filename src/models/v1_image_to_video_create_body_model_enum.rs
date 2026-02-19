/// The AI model to use for video generation.
/// * `default`: Our recommended model for general use (Kling 2.5 Audio). Note: For backward compatibility, if you use `default` and `end_seconds` > 10, we'll fall back to kling-1.6.
/// * `ltx-2`: Great for fast iteration with audio, lip-sync, and expressive faces
/// * `seedance`: Great for fast iteration and start/end frame
/// * `kling-2.5`: Great for motion, action, and camera control
/// * `kling-3.0`: Great for cinematic, multi-scene storytelling with control
/// * `sora-2`: Great for story-telling, dialogue & creativity
/// * `veo3.1`: Great for realism, polish, & prompt adherence
/// * `kling-1.6`: Great for dependable clips with smooth motion
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
    #[serde(rename = "sora-2")]
    Sora2,
    #[serde(rename = "veo3.1")]
    Veo31,
    #[serde(rename = "veo3.1-audio")]
    Veo31Audio,
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
            V1ImageToVideoCreateBodyModelEnum::Sora2 => "sora-2",
            V1ImageToVideoCreateBodyModelEnum::Veo31 => "veo3.1",
            V1ImageToVideoCreateBodyModelEnum::Veo31Audio => "veo3.1-audio",
        };
        write!(f, "{}", str_val)
    }
}
