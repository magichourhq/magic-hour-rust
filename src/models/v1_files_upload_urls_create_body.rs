/// V1FilesUploadUrlsCreateBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1FilesUploadUrlsCreateBody {
    /// The list of assets to upload. The response array will match the order of items in the request body.
    pub items: Vec<crate::models::V1FilesUploadUrlsCreateBodyItemsItem>,
}
