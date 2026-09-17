/// V1SavedItemsListResponseItemsItem
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1SavedItemsListResponseItemsItem {
    pub assets: Vec<crate::models::V1SavedItemsListResponseItemsItemAssetsItem>,
    /// Unique ID of the saved item.
    pub id: String,
    /// User-provided name of the saved item.
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub name: Option<String>,
    /// Saved item type.
    #[serde(rename = "type")]
    pub type_: crate::models::V1SavedItemsListResponseItemsItemTypeEnum,
}
