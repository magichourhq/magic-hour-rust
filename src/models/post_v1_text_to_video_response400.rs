/// The request is invalid
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1TextToVideoResponse400 {
    pub message: String,
}
