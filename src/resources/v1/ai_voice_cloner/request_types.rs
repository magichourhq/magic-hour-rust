/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    /// Provide the assets for voice cloning.
    pub assets: crate::models::V1AiVoiceClonerCreateBodyAssets,
    /// Give your audio a custom name for easy identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub style: crate::models::V1AiVoiceClonerCreateBodyStyle,
}
