/// Success
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1FilesUploadUrlsResponse {
    pub items: Vec<crate::models::PostV1FilesUploadUrlsResponseItemsItem>,
}
