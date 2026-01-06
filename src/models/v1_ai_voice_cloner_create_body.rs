/// V1AiVoiceClonerCreateBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiVoiceClonerCreateBody {
    /// Provide the assets for voice cloning.
    pub assets: crate::models::V1AiVoiceClonerCreateBodyAssets,
    /// Give your audio a custom name for easy identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub style: crate::models::V1AiVoiceClonerCreateBodyStyle,
}
