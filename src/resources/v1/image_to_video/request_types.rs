/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    /// Provide the assets for image-to-video.
    pub assets: crate::models::PostV1ImageToVideoBodyAssets,
    /// The total duration of the output video in seconds.
    pub end_seconds: f64,
    /// The height of the input video. This value will help determine the final orientation of the output video. The output video resolution may not match the input.
    pub height: i64,
    /// The name of video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub style: crate::models::PostV1ImageToVideoBodyStyle,
    /// The width of the input video. This value will help determine the final orientation of the output video. The output video resolution may not match the input.
    pub width: i64,
}
