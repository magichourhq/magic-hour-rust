/// Requested resource is not found
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiImageGeneratorcreateResponse404 {
    pub message: crate::models::V1AiImageGeneratorcreateResponse404MessageEnum,
}
