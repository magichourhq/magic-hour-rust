/// Success
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1VideoProjectsgetResponse {
    pub created_at: String,
    /// Deprecated: Please use `.downloads` instead. The download url and expiration date of the video project
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub download: Option<crate::models::V1VideoProjectsgetResponseDownload>,
    pub downloads: Vec<crate::models::V1VideoProjectsgetResponseDownloadsItem>,
    /// Indicates whether the resource is deleted
    pub enabled: bool,
    /// The end time of the input video in seconds
    pub end_seconds: f64,
    /// In the case of an error, this object will contain the error encountered during video render
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub error: Option<crate::models::V1VideoProjectsgetResponseError>,
    /// Frame rate of the video. If the status is not 'complete', the frame rate is an estimate and will be adjusted when the video completes.
    pub fps: f64,
    /// The height of the final output video. The maximum height depends on your subscription. Please refer to our [pricing page](https://magichour.ai/pricing) for more details
    pub height: i64,
    /// Unique ID of the video. This value can be used in the [get video project API](/api/tag/video-projects/get/v1/video-projects/{id}) to fetch additional details such as status
    pub id: String,
    /// The name of the video.
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub name: Option<String>,
    /// The start time of the input video in seconds
    pub start_seconds: f64,
    /// The status of the video.
    pub status: crate::models::V1VideoProjectsgetResponseStatusEnum,
    /// The amount of frames used to generate the video. If the status is not 'complete', the cost is an estimate and will be adjusted when the video completes.
    pub total_frame_cost: i64,
    #[serde(rename = "type")]
    pub type_field: crate::models::V1VideoProjectsgetResponseTypeEnum,
    /// The width of the final output video. The maximum width depends on your subscription. Please refer to our [pricing page](https://magichour.ai/pricing) for more details
    pub width: i64,
}
