/// V1VoicesListResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1VoicesListResponse {
    /// Cursor for the next page, or null when there are no more voices.
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub next_cursor: Option<String>,
    pub voices: Vec<crate::models::V1VoicesListResponseVoicesItem>,
}
