/// The request is invalid
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1AiHeadshotGeneratorResponse400 {
    pub message: String,
}
