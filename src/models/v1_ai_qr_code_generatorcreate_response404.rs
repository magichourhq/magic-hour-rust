/// Requested resource is not found
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiQrCodeGeneratorcreateResponse404 {
    pub message: crate::models::V1AiQrCodeGeneratorcreateResponse404MessageEnum,
}
