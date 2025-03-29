/// Success
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1FaceSwapCreateResponse {
    /// Estimated cost of the video in terms of number of frames needed to render the video. Frames will be adjusted when the video completes
    pub estimated_frame_cost: i64,
    /// Unique ID of the image. This value can be used in the [get image project API](https://docs.magichour.ai/api-reference/image-projects/get-image-details) to fetch additional details such as status
    pub id: String,
}
