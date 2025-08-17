/// Success
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1FilesUploadUrlsCreateResponse {
    /// The list of upload URLs and file paths for the assets. The response array will match the order of items in the request body. Refer to the [Input Files Guide](/integration/input-files) for more details.
    pub items: Vec<crate::models::V1FilesUploadUrlsCreateResponseItemsItem>,
}
