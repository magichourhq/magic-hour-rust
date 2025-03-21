/// PostV1FilesUploadUrlsBodyItemsItem
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1FilesUploadUrlsBodyItemsItem {
    /// the extension of the file to upload. Do not include the dot (.) before the extension.
    pub extension: String,
    /// The type of asset to upload
    #[serde(rename = "type")]
    pub type_field: crate::models::PostV1FilesUploadUrlsBodyItemsItemTypeEnum,
}
