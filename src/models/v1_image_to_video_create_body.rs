/// V1ImageToVideoCreateBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1ImageToVideoCreateBody {
    /// Provide the assets for image-to-video.
    pub assets: crate::models::V1ImageToVideoCreateBodyAssets,
    /// The total duration of the output video in seconds.
    pub end_seconds: f64,
    /// This field does not affect the output video's resolution. The video's orientation will match that of the input image.
    ///
    /// It is retained solely for backward compatibility and will be deprecated in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// The name of video. This value is mainly used for your own identification of the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Controls the output video resolution. Defaults to `720p` if not specified.
    ///
    /// 480p and 720p are available on Creator, Pro, or Business tiers. However, 1080p require Pro or Business tier.
    ///
    /// **Options:**
    /// - `480p` - Supports only 5 or 10 second videos. Output: 24fps. Cost: 120 credits per 5 seconds.
    /// - `720p` - Supports videos between 5-60 seconds. Output: 30fps. Cost: 300 credits per 5 seconds.
    /// - `1080p` - Supports videos between 5-60 seconds. Output: 30fps. Cost: 600 credits per 5 seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<crate::models::V1ImageToVideoCreateBodyResolutionEnum>,
    /// Attributed used to dictate the style of the output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<crate::models::V1ImageToVideoCreateBodyStyle>,
    /// This field does not affect the output video's resolution. The video's orientation will match that of the input image.
    ///
    /// It is retained solely for backward compatibility and will be deprecated in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}
