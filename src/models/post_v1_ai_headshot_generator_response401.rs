/// The request is not properly authenticated
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1AiHeadshotGeneratorResponse401 {
    pub message: crate::models::PostV1AiHeadshotGeneratorResponse401MessageEnum,
}
