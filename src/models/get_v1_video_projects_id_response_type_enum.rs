/// GetV1VideoProjectsIdResponseTypeEnum
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum GetV1VideoProjectsIdResponseTypeEnum {
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
impl std::fmt::Display for GetV1VideoProjectsIdResponseTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetV1VideoProjectsIdResponseTypeEnum::Animation => "ANIMATION",
            GetV1VideoProjectsIdResponseTypeEnum::AutoSubtitle => "AUTO_SUBTITLE",
            GetV1VideoProjectsIdResponseTypeEnum::FaceSwap => "FACE_SWAP",
            GetV1VideoProjectsIdResponseTypeEnum::ImageToVideo => "IMAGE_TO_VIDEO",
            GetV1VideoProjectsIdResponseTypeEnum::LipSync => "LIP_SYNC",
            GetV1VideoProjectsIdResponseTypeEnum::TextToVideo => "TEXT_TO_VIDEO",
            GetV1VideoProjectsIdResponseTypeEnum::VideoToVideo => "VIDEO_TO_VIDEO",
        };
        write!(f, "{}", str_val)
    }
}
