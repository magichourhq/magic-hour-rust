/// The request is invalid
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1AiImageGeneratorResponse400 {
    pub message: String,
}
