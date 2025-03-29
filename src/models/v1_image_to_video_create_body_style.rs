/// V1ImageToVideoCreateBodyStyle
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1ImageToVideoCreateBodyStyle {
    /// High Quality mode enhances detail, sharpness, and realism, making it ideal for portraits, animals, and intricate landscapes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_quality: Option<bool>,
    /// The prompt used for the video.
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub prompt: Option<String>,
}
