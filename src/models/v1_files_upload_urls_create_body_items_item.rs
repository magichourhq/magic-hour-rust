/// V1FilesUploadUrlsCreateBodyItemsItem
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1FilesUploadUrlsCreateBodyItemsItem {
    /// The extension of the file to upload. Do not include the dot (.) before the extension. Possible extensions are mp4,m4v,mov,webm,mp3,wav,aac,flac,webm,png,jpg,jpeg,heic,webp,avif,jp2,tiff,bmp,gif,webp,webm
    pub extension: String,
    /// The type of asset to upload. Possible types are video, audio, image
    #[serde(rename = "type")]
    pub type_: crate::models::V1FilesUploadUrlsCreateBodyItemsItemTypeEnum,
}
