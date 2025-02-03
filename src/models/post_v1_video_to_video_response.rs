/// Success
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1VideoToVideoResponse {
    /// Estimated cost of the video in terms of number of frames needed to render the video. Frames will be adjusted when the video completes
    pub estimated_frame_cost: i64,
    /// Unique ID of the video. This value can be used in the [get video project API](/api/tag/video-projects/get/v1/video-projects/{id}) to fetch additional details such as status
    pub id: String,
}
