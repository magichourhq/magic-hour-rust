/// V1AiVoiceClonerCreateBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiVoiceClonerCreateBody {
    /// Provide the assets for voice cloning.
    pub assets: crate::models::V1AiVoiceClonerCreateBodyAssets,
    /// The name of audio. This value is mainly used for your own identification of the audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub style: crate::models::V1AiVoiceClonerCreateBodyStyle,
}
