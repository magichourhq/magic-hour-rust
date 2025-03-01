/// The type of asset to upload
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum PostV1FilesUploadUrlsBodyItemsItemTypeEnum {
    #[default]
    #[serde(rename = "audio")]
    Audio,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "video")]
    Video,
}
impl std::fmt::Display for PostV1FilesUploadUrlsBodyItemsItemTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostV1FilesUploadUrlsBodyItemsItemTypeEnum::Audio => "audio",
            PostV1FilesUploadUrlsBodyItemsItemTypeEnum::Image => "image",
            PostV1FilesUploadUrlsBodyItemsItemTypeEnum::Video => "video",
        };
        write!(f, "{}", str_val)
    }
}
