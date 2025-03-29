/// Success
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1FilesUploadUrlsCreateResponse {
    pub items: Vec<crate::models::V1FilesUploadUrlsCreateResponseItemsItem>,
}
