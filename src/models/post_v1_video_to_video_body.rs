/// PostV1VideoToVideoBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1VideoToVideoBody {
    /// Provide the assets for video-to-video. For video, The `video_source` field determines whether `video_file_path` or `youtube_url` field is used
    pub assets: crate::models::PostV1VideoToVideoBodyAssets,
    /// The end time of the input video in seconds
    pub end_seconds: f64,
    /// Determines whether the resulting video will have the same frame per second as the original video, or half.
    /// * `FULL` - the result video will have the same FPS as the input video
    /// * `HALF` - the result video will have half the FPS as the input video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fps_resolution: Option<crate::models::PostV1VideoToVideoBodyFpsResolutionEnum>,
    /// The height of the final output video. Must be divisible by 64. The maximum height depends on your subscription. Please refer to our [pricing page](https://magichour.ai/pricing) for more details
    pub height: i64,
    /// The name of video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The start time of the input video in seconds
    pub start_seconds: f64,
    pub style: crate::models::PostV1VideoToVideoBodyStyle,
    /// The width of the final output video. Must be divisible by 64. The maximum width depends on your subscription. Please refer to our [pricing page](https://magichour.ai/pricing) for more details
    pub width: i64,
}
