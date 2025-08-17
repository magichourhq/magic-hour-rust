/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    /// Provide the assets for animation.
    pub assets: crate::models::V1AnimationCreateBodyAssets,
    /// This value determines the duration of the output video.
    pub end_seconds: f64,
    /// The desire output video frame rate
    pub fps: f64,
    /// The height of the final output video. The maximum height depends on your subscription. Please refer to our [pricing page](https://magichour.ai/pricing) for more details
    pub height: i64,
    /// The name of video. This value is mainly used for your own identification of the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Defines the style of the output video
    pub style: crate::models::V1AnimationCreateBodyStyle,
    /// The width of the final output video. The maximum width depends on your subscription. Please refer to our [pricing page](https://magichour.ai/pricing) for more details
    pub width: i64,
}
