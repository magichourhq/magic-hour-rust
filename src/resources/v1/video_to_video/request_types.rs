/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    /// Provide the assets for video-to-video. For video, The `video_source` field determines whether `video_file_path` or `youtube_url` field is used
    pub assets: crate::models::V1VideoToVideoCreateBodyAssets,
    /// The end time of the input video in seconds
    pub end_seconds: f64,
    /// Determines whether the resulting video will have the same frame per second as the original video, or half.
    /// * `FULL` - the result video will have the same FPS as the input video
    /// * `HALF` - the result video will have half the FPS as the input video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fps_resolution: Option<crate::models::V1VideoToVideoCreateBodyFpsResolutionEnum>,
    /// Used to determine the dimensions of the output video.
    ///
    /// * If height is provided, width will also be required. The larger value between width and height will be used to determine the maximum output resolution while maintaining the original aspect ratio.
    /// * If both height and width are omitted, the video will be resized according to your subscription's maximum resolution, while preserving aspect ratio.
    ///
    /// Note: if the video's original resolution is less than the maximum, the video will not be resized.
    ///
    /// See our [pricing page](https://magichour.ai/pricing) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// The name of video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The start time of the input video in seconds
    pub start_seconds: f64,
    pub style: crate::models::V1VideoToVideoCreateBodyStyle,
    /// Used to determine the dimensions of the output video.
    ///
    /// * If width is provided, height will also be required. The larger value between width and height will be used to determine the maximum output resolution while maintaining the original aspect ratio.
    /// * If both height and width are omitted, the video will be resized according to your subscription's maximum resolution, while preserving aspect ratio.
    ///
    /// Note: if the video's original resolution is less than the maximum, the video will not be resized.
    ///
    /// See our [pricing page](https://magichour.ai/pricing) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}
