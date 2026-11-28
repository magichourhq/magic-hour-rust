/// The create endpoint to price, named by its path. For example `ai-image-editor` for `POST /v1/ai-image-editor`.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1EstimateImageListToolEnum {
    #[default]
    #[serde(rename = "ai-clothes-changer")]
    AiClothesChanger,
    #[serde(rename = "ai-face-editor")]
    AiFaceEditor,
    #[serde(rename = "ai-gif-generator")]
    AiGifGenerator,
    #[serde(rename = "ai-headshot-generator")]
    AiHeadshotGenerator,
    #[serde(rename = "ai-image-editor")]
    AiImageEditor,
    #[serde(rename = "ai-image-generator")]
    AiImageGenerator,
    #[serde(rename = "ai-image-upscaler")]
    AiImageUpscaler,
    #[serde(rename = "ai-meme-generator")]
    AiMemeGenerator,
    #[serde(rename = "ai-qr-code-generator")]
    AiQrCodeGenerator,
    #[serde(rename = "body-swap")]
    BodySwap,
    #[serde(rename = "face-swap-photo")]
    FaceSwapPhoto,
    #[serde(rename = "head-swap")]
    HeadSwap,
    #[serde(rename = "image-background-remover")]
    ImageBackgroundRemover,
    #[serde(rename = "photo-colorizer")]
    PhotoColorizer,
}
impl std::fmt::Display for V1EstimateImageListToolEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1EstimateImageListToolEnum::AiClothesChanger => "ai-clothes-changer",
            V1EstimateImageListToolEnum::AiFaceEditor => "ai-face-editor",
            V1EstimateImageListToolEnum::AiGifGenerator => "ai-gif-generator",
            V1EstimateImageListToolEnum::AiHeadshotGenerator => "ai-headshot-generator",
            V1EstimateImageListToolEnum::AiImageEditor => "ai-image-editor",
            V1EstimateImageListToolEnum::AiImageGenerator => "ai-image-generator",
            V1EstimateImageListToolEnum::AiImageUpscaler => "ai-image-upscaler",
            V1EstimateImageListToolEnum::AiMemeGenerator => "ai-meme-generator",
            V1EstimateImageListToolEnum::AiQrCodeGenerator => "ai-qr-code-generator",
            V1EstimateImageListToolEnum::BodySwap => "body-swap",
            V1EstimateImageListToolEnum::FaceSwapPhoto => "face-swap-photo",
            V1EstimateImageListToolEnum::HeadSwap => "head-swap",
            V1EstimateImageListToolEnum::ImageBackgroundRemover => {
                "image-background-remover"
            }
            V1EstimateImageListToolEnum::PhotoColorizer => "photo-colorizer",
        };
        write!(f, "{}", str_val)
    }
}
