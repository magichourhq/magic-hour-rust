/// V1ImageProjectsGetResponseTypeEnum
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1ImageProjectsGetResponseTypeEnum {
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
impl std::fmt::Display for V1ImageProjectsGetResponseTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1ImageProjectsGetResponseTypeEnum::AiHeadshot => "AI_HEADSHOT",
            V1ImageProjectsGetResponseTypeEnum::AiImage => "AI_IMAGE",
            V1ImageProjectsGetResponseTypeEnum::BackgroundRemover => "BACKGROUND_REMOVER",
            V1ImageProjectsGetResponseTypeEnum::ClothesChanger => "CLOTHES_CHANGER",
            V1ImageProjectsGetResponseTypeEnum::FaceSwap => "FACE_SWAP",
            V1ImageProjectsGetResponseTypeEnum::ImageUpscaler => "IMAGE_UPSCALER",
            V1ImageProjectsGetResponseTypeEnum::PhotoEditor => "PHOTO_EDITOR",
            V1ImageProjectsGetResponseTypeEnum::QrCode => "QR_CODE",
        };
        write!(f, "{}", str_val)
    }
}
