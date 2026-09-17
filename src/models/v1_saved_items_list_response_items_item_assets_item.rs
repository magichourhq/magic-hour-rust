/// V1SavedItemsListResponseItemsItemAssetsItem
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1SavedItemsListResponseItemsItemAssetsItem {
    /// Durable asset path. Pass it to a compatible API asset field without uploading it again.
    pub file_path: String,
    /// Whether this asset is the saved item's primary asset.
    pub is_primary: bool,
    /// Media type of the asset.
    pub media_kind: crate::models::V1SavedItemsListResponseItemsItemAssetsItemMediaKindEnum,
    /// Signed URL for previewing or downloading the asset. Expires after 24 hours.
    pub url: String,
    /// When the signed URL expires. The saved asset and file_path do not expire.
    pub url_expires_at: String,
}
