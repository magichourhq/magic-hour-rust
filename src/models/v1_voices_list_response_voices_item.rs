/// V1VoicesListResponseVoicesItem
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1VoicesListResponseVoicesItem {
    /// URL of the voice's avatar image, or null when the voice has no avatar.
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub avatar_url: Option<String>,
    /// Unique ID of the voice. Pass it as `style.voice_name` when generating speech. Names are not unique, so use the ID.
    pub id: String,
    /// Display name of the voice.
    pub name: String,
    /// URL of an audio sample of the voice.
    pub preview_url: String,
}
