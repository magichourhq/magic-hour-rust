/// The request is not properly authenticated
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1AiQrCodeGeneratorResponse401 {
    pub message: crate::models::PostV1AiQrCodeGeneratorResponse401MessageEnum,
}
