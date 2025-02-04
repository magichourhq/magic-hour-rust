/// V1VideoProjectsgetResponseTypeEnum
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1VideoProjectsgetResponseTypeEnum {
    #[default]
    #[serde(rename = "ANIMATION")]
    Animation,
    #[serde(rename = "AUTO_SUBTITLE")]
    AutoSubtitle,
    #[serde(rename = "FACE_SWAP")]
    FaceSwap,
    #[serde(rename = "IMAGE_TO_VIDEO")]
    ImageToVideo,
    #[serde(rename = "LIP_SYNC")]
    LipSync,
    #[serde(rename = "TEXT_TO_VIDEO")]
    TextToVideo,
    #[serde(rename = "VIDEO_TO_VIDEO")]
    VideoToVideo,
}
impl std::fmt::Display for V1VideoProjectsgetResponseTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1VideoProjectsgetResponseTypeEnum::Animation => "ANIMATION",
            V1VideoProjectsgetResponseTypeEnum::AutoSubtitle => "AUTO_SUBTITLE",
            V1VideoProjectsgetResponseTypeEnum::FaceSwap => "FACE_SWAP",
            V1VideoProjectsgetResponseTypeEnum::ImageToVideo => "IMAGE_TO_VIDEO",
            V1VideoProjectsgetResponseTypeEnum::LipSync => "LIP_SYNC",
            V1VideoProjectsgetResponseTypeEnum::TextToVideo => "TEXT_TO_VIDEO",
            V1VideoProjectsgetResponseTypeEnum::VideoToVideo => "VIDEO_TO_VIDEO",
        };
        write!(f, "{}", str_val)
    }
}
