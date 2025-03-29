/// Requested resource is not found
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiImageUpscalerCreateResponse404 {
    pub message: crate::models::V1AiImageUpscalerCreateResponse404MessageEnum,
}
