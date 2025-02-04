/// The request is not properly authenticated
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiImageGeneratorcreateResponse401 {
    pub message: crate::models::V1AiImageGeneratorcreateResponse401MessageEnum,
}
