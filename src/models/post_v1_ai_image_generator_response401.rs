/// The request is not properly authenticated
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1AiImageGeneratorResponse401 {
    pub message: crate::models::PostV1AiImageGeneratorResponse401MessageEnum,
}
