/// Success
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1ImageProjectsGetResponse {
    pub created_at: String,
    pub downloads: Vec<crate::models::V1ImageProjectsGetResponseDownloadsItem>,
    /// Indicates whether the resource is deleted
    pub enabled: bool,
    /// In the case of an error, this object will contain the error encountered during video render
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub error: Option<crate::models::V1ImageProjectsGetResponseError>,
    /// Unique ID of the image. This value can be used in the [get image project API](https://docs.magichour.ai/api-reference/image-projects/get-image-details) to fetch additional details such as status
    pub id: String,
    /// Number of images generated
    pub image_count: i64,
    /// The name of the image.
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub name: Option<String>,
    /// The status of the image.
    pub status: crate::models::V1ImageProjectsGetResponseStatusEnum,
    /// The amount of frames used to generate the image.
    pub total_frame_cost: i64,
    /// The type of the image project. Possible values are AI_HEADSHOT, AI_IMAGE, IMAGE_UPSCALER, FACE_SWAP, PHOTO_EDITOR, QR_CODE, BACKGROUND_REMOVER, CLOTHES_CHANGER, AI_MEME, FACE_EDITOR, PHOTO_COLORIZER
    #[serde(rename = "type")]
    pub type_: String,
}
