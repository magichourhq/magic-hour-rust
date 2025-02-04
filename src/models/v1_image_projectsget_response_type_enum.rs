/// V1ImageProjectsgetResponseTypeEnum
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1ImageProjectsgetResponseTypeEnum {
    #[default]
    #[serde(rename = "AI_HEADSHOT")]
    AiHeadshot,
    #[serde(rename = "AI_IMAGE")]
    AiImage,
    #[serde(rename = "BACKGROUND_REMOVER")]
    BackgroundRemover,
    #[serde(rename = "CLOTHES_CHANGER")]
    ClothesChanger,
    #[serde(rename = "FACE_SWAP")]
    FaceSwap,
    #[serde(rename = "IMAGE_UPSCALER")]
    ImageUpscaler,
    #[serde(rename = "PHOTO_EDITOR")]
    PhotoEditor,
    #[serde(rename = "QR_CODE")]
    QrCode,
}
impl std::fmt::Display for V1ImageProjectsgetResponseTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1ImageProjectsgetResponseTypeEnum::AiHeadshot => "AI_HEADSHOT",
            V1ImageProjectsgetResponseTypeEnum::AiImage => "AI_IMAGE",
            V1ImageProjectsgetResponseTypeEnum::BackgroundRemover => "BACKGROUND_REMOVER",
            V1ImageProjectsgetResponseTypeEnum::ClothesChanger => "CLOTHES_CHANGER",
            V1ImageProjectsgetResponseTypeEnum::FaceSwap => "FACE_SWAP",
            V1ImageProjectsgetResponseTypeEnum::ImageUpscaler => "IMAGE_UPSCALER",
            V1ImageProjectsgetResponseTypeEnum::PhotoEditor => "PHOTO_EDITOR",
            V1ImageProjectsgetResponseTypeEnum::QrCode => "QR_CODE",
        };
        write!(f, "{}", str_val)
    }
}
