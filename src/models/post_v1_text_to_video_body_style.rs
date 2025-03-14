/// PostV1TextToVideoBodyStyle
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1TextToVideoBodyStyle {
    /// The prompt used for the video.
    pub prompt: String,
}
