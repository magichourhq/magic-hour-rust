/// The request is invalid
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1AiQrCodeGeneratorResponse400 {
    pub message: String,
}
