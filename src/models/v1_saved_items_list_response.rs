/// V1SavedItemsListResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1SavedItemsListResponse {
    pub items: Vec<crate::models::V1SavedItemsListResponseItemsItem>,
    /// Cursor for the next page, or null when there are no more saved items.
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub next_cursor: Option<String>,
}
