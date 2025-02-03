/// Optionally add an audio source if you'd like to incorporate audio into your video
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum PostV1AnimationBodyAssetsAudioSourceEnum {
    #[default]
    #[serde(rename = "file")]
    File,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "youtube")]
    Youtube,
}
impl std::fmt::Display for PostV1AnimationBodyAssetsAudioSourceEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostV1AnimationBodyAssetsAudioSourceEnum::File => "file",
            PostV1AnimationBodyAssetsAudioSourceEnum::None => "none",
            PostV1AnimationBodyAssetsAudioSourceEnum::Youtube => "youtube",
        };
        write!(f, "{}", str_val)
    }
}
