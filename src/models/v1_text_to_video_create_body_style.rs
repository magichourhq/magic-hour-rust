/// V1TextToVideoCreateBodyStyle
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1TextToVideoCreateBodyStyle {
    /// The prompt used for the video.
    pub prompt: String,
}
