/// The request is not properly authenticated
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1AiClothesChangerResponse401 {
    pub message: crate::models::PostV1AiClothesChangerResponse401MessageEnum,
}
