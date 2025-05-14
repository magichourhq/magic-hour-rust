/// Success
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1VideoProjectsGetResponse {
    pub created_at: String,
    /// The amount of credits deducted from your account to generate the video. If the status is not 'complete', this value is an estimate and may be adjusted upon completion based on the actual FPS of the output video.
    ///
    /// If video generation fails, credits will be refunded, and this field will be updated to include the refund.
    pub credits_charged: i64,
    /// Deprecated: Please use `.downloads` instead. The download url and expiration date of the video project
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub download: Option<crate::models::V1VideoProjectsGetResponseDownload>,
    pub downloads: Vec<crate::models::V1VideoProjectsGetResponseDownloadsItem>,
    /// Indicates whether the resource is deleted
    pub enabled: bool,
    /// The end time of the input video in seconds
    pub end_seconds: f64,
    /// In the case of an error, this object will contain the error encountered during video render
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub error: Option<crate::models::V1VideoProjectsGetResponseError>,
    /// Frame rate of the video. If the status is not 'complete', the frame rate is an estimate and will be adjusted when the video completes.
    pub fps: f64,
    /// The height of the final output video. A value of -1 indicates the height can be ignored.
    pub height: i64,
    /// Unique ID of the video. This value can be used in the [get video project API](https://docs.magichour.ai/api-reference/video-projects/get-video-details) to fetch additional details such as status
    pub id: String,
    /// The name of the video.
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub name: Option<String>,
    /// The start time of the input video in seconds
    pub start_seconds: f64,
    /// The status of the video.
    pub status: crate::models::V1VideoProjectsGetResponseStatusEnum,
    /// Deprecated: Previously represented the number of frames (original name of our credit system) used for video generation. Use 'credits_charged' instead.
    ///
    /// The amount of frames used to generate the video. If the status is not 'complete', the cost is an estimate and will be adjusted when the video completes.
    pub total_frame_cost: i64,
    /// The type of the video project. Possible values are ANIMATION, IMAGE_TO_VIDEO, VIDEO_TO_VIDEO, TEXT_TO_VIDEO, FACE_SWAP, LIP_SYNC, AUTO_SUBTITLE, TALKING_PHOTO
    #[serde(rename = "type")]
    pub type_: String,
    /// The width of the final output video. A value of -1 indicates the width can be ignored.
    pub width: i64,
}
