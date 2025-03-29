/// V1VideoProjectsGetResponseTypeEnum
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1VideoProjectsGetResponseTypeEnum {
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
    #[serde(rename = "TALKING_PHOTO")]
    TalkingPhoto,
    #[serde(rename = "TEXT_TO_VIDEO")]
    TextToVideo,
    #[serde(rename = "VIDEO_TO_VIDEO")]
    VideoToVideo,
}
impl std::fmt::Display for V1VideoProjectsGetResponseTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1VideoProjectsGetResponseTypeEnum::Animation => "ANIMATION",
            V1VideoProjectsGetResponseTypeEnum::AutoSubtitle => "AUTO_SUBTITLE",
            V1VideoProjectsGetResponseTypeEnum::FaceSwap => "FACE_SWAP",
            V1VideoProjectsGetResponseTypeEnum::ImageToVideo => "IMAGE_TO_VIDEO",
            V1VideoProjectsGetResponseTypeEnum::LipSync => "LIP_SYNC",
            V1VideoProjectsGetResponseTypeEnum::TalkingPhoto => "TALKING_PHOTO",
            V1VideoProjectsGetResponseTypeEnum::TextToVideo => "TEXT_TO_VIDEO",
            V1VideoProjectsGetResponseTypeEnum::VideoToVideo => "VIDEO_TO_VIDEO",
        };
        write!(f, "{}", str_val)
    }
}
