/// Success
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetV1ImageProjectsIdResponse {
    pub created_at: String,
    pub downloads: Vec<crate::models::GetV1ImageProjectsIdResponseDownloadsItem>,
    /// Indicates whether the resource is deleted
    pub enabled: bool,
    /// In the case of an error, this object will contain the error encountered during video render
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub error: Option<crate::models::GetV1ImageProjectsIdResponseError>,
    /// Unique ID of the image. This value can be used in the [get image project API](https://docs.magichour.ai/api-reference/image-projects/get-image-details) to fetch additional details such as status
    pub id: String,
    /// Number of images generated
    pub image_count: i64,
    /// The name of the image.
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub name: Option<String>,
    /// The status of the image.
    pub status: crate::models::GetV1ImageProjectsIdResponseStatusEnum,
    /// The amount of frames used to generate the image.
    pub total_frame_cost: i64,
    #[serde(rename = "type")]
    pub type_field: crate::models::GetV1ImageProjectsIdResponseTypeEnum,
}
