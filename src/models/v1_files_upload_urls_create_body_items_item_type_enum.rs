/// The type of asset to upload
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1FilesUploadUrlsCreateBodyItemsItemTypeEnum {
    #[default]
    #[serde(rename = "audio")]
    Audio,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "video")]
    Video,
}
impl std::fmt::Display for V1FilesUploadUrlsCreateBodyItemsItemTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1FilesUploadUrlsCreateBodyItemsItemTypeEnum::Audio => "audio",
            V1FilesUploadUrlsCreateBodyItemsItemTypeEnum::Image => "image",
            V1FilesUploadUrlsCreateBodyItemsItemTypeEnum::Video => "video",
        };
        write!(f, "{}", str_val)
    }
}
