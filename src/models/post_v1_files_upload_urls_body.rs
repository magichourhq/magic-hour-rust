/// PostV1FilesUploadUrlsBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1FilesUploadUrlsBody {
    pub items: Vec<crate::models::PostV1FilesUploadUrlsBodyItemsItem>,
}
