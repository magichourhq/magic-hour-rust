/// The request is not properly authenticated
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1TextToVideoResponse401 {
    pub message: crate::models::PostV1TextToVideoResponse401MessageEnum,
}
