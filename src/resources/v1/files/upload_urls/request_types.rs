/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    /// The list of assets to upload. The response array will match the order of items in the request body.
    pub items: Vec<crate::models::V1FilesUploadUrlsCreateBodyItemsItem>,
}
