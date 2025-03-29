/// V1FilesUploadUrlsCreateBodyItemsItem
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1FilesUploadUrlsCreateBodyItemsItem {
    /// the extension of the file to upload. Do not include the dot (.) before the extension.
    pub extension: String,
    /// The type of asset to upload
    #[serde(rename = "type")]
    pub type_: crate::models::V1FilesUploadUrlsCreateBodyItemsItemTypeEnum,
}
