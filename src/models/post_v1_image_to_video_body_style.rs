/// Generated by Sideko (sideko.dev)
/// PostV1ImageToVideoBodyStyle
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1ImageToVideoBodyStyle {
    /// The prompt used for the video.
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub prompt: Option<String>,
}
