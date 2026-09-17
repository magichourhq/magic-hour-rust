/// ListRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListRequest {
    /// Opaque pagination cursor from the previous response's next_cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Maximum number of saved items to return. Defaults to 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Only return saved items of this type.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<crate::models::V1SavedItemsListTypeEnum>,
}
