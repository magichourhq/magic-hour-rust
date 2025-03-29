/// Optionally add an audio source if you'd like to incorporate audio into your video
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AnimationCreateBodyAssetsAudioSourceEnum {
    #[default]
    #[serde(rename = "file")]
    File,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "youtube")]
    Youtube,
}
impl std::fmt::Display for V1AnimationCreateBodyAssetsAudioSourceEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AnimationCreateBodyAssetsAudioSourceEnum::File => "file",
            V1AnimationCreateBodyAssetsAudioSourceEnum::None => "none",
            V1AnimationCreateBodyAssetsAudioSourceEnum::Youtube => "youtube",
        };
        write!(f, "{}", str_val)
    }
}
