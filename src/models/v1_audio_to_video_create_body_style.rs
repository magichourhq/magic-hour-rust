/// Attributes used to dictate the style of the output
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AudioToVideoCreateBodyStyle {
    /// Prompt to guide the visual style of the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
}
