/// The create endpoint to price, named by its path. For example `image-to-video` for `POST /v1/image-to-video`.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1EstimateVideoListToolEnum {
    #[default]
    #[serde(rename = "ai-ugc-ad")]
    AiUgcAd,
    #[serde(rename = "ai-video-editor")]
    AiVideoEditor,
    #[serde(rename = "ai-video-translator")]
    AiVideoTranslator,
    #[serde(rename = "ai-video-upscaler")]
    AiVideoUpscaler,
    #[serde(rename = "audio-to-video")]
    AudioToVideo,
    #[serde(rename = "auto-subtitle-generator")]
    AutoSubtitleGenerator,
    #[serde(rename = "character-replace")]
    CharacterReplace,
    #[serde(rename = "face-swap")]
    FaceSwap,
    #[serde(rename = "image-to-video")]
    ImageToVideo,
    #[serde(rename = "text-to-video")]
    TextToVideo,
}
impl std::fmt::Display for V1EstimateVideoListToolEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1EstimateVideoListToolEnum::AiUgcAd => "ai-ugc-ad",
            V1EstimateVideoListToolEnum::AiVideoEditor => "ai-video-editor",
            V1EstimateVideoListToolEnum::AiVideoTranslator => "ai-video-translator",
            V1EstimateVideoListToolEnum::AiVideoUpscaler => "ai-video-upscaler",
            V1EstimateVideoListToolEnum::AudioToVideo => "audio-to-video",
            V1EstimateVideoListToolEnum::AutoSubtitleGenerator => {
                "auto-subtitle-generator"
            }
            V1EstimateVideoListToolEnum::CharacterReplace => "character-replace",
            V1EstimateVideoListToolEnum::FaceSwap => "face-swap",
            V1EstimateVideoListToolEnum::ImageToVideo => "image-to-video",
            V1EstimateVideoListToolEnum::TextToVideo => "text-to-video",
        };
        write!(f, "{}", str_val)
    }
}
