/// GetV1ImageProjectsIdResponseTypeEnum
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum GetV1ImageProjectsIdResponseTypeEnum {
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
impl std::fmt::Display for GetV1ImageProjectsIdResponseTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetV1ImageProjectsIdResponseTypeEnum::AiHeadshot => "AI_HEADSHOT",
            GetV1ImageProjectsIdResponseTypeEnum::AiImage => "AI_IMAGE",
            GetV1ImageProjectsIdResponseTypeEnum::BackgroundRemover => {
                "BACKGROUND_REMOVER"
            }
            GetV1ImageProjectsIdResponseTypeEnum::ClothesChanger => "CLOTHES_CHANGER",
            GetV1ImageProjectsIdResponseTypeEnum::FaceSwap => "FACE_SWAP",
            GetV1ImageProjectsIdResponseTypeEnum::ImageUpscaler => "IMAGE_UPSCALER",
            GetV1ImageProjectsIdResponseTypeEnum::PhotoEditor => "PHOTO_EDITOR",
            GetV1ImageProjectsIdResponseTypeEnum::QrCode => "QR_CODE",
        };
        write!(f, "{}", str_val)
    }
}
