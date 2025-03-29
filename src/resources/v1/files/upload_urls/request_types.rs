/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    pub items: Vec<crate::models::V1FilesUploadUrlsCreateBodyItemsItem>,
}
