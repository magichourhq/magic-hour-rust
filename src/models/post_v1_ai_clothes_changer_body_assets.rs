/// Provide the assets for clothes changer
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1AiClothesChangerBodyAssets {
    /// The image of the outfit. This value can be either the `file_path` field from the response of the [upload urls API](/docs/api/tag/files/post/v1/files/upload-urls), or the url of the file..
    pub garment_file_path: String,
    pub garment_type: crate::models::PostV1AiClothesChangerBodyAssetsGarmentTypeEnum,
    /// The image with the person. This value can be either the `file_path` field from the response of the [upload urls API](/docs/api/tag/files/post/v1/files/upload-urls), or the url of the file..
    pub person_file_path: String,
}
