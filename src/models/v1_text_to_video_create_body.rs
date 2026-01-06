/// V1TextToVideoCreateBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1TextToVideoCreateBody {
    /// The total duration of the output video in seconds.
    ///
    /// The value must be greater than or equal to 5 seconds and less than or equal to 60 seconds.
    ///
    /// Note: For 480p resolution, the value must be either 5 or 10.
    pub end_seconds: f64,
    /// Give your video a custom name for easy identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Determines the orientation of the output video
    pub orientation: crate::models::V1TextToVideoCreateBodyOrientationEnum,
    /// Controls the output video resolution. Defaults to `720p` if not specified.
    ///
    /// 480p and 720p are available on Creator, Pro, or Business tiers. However, 1080p require Pro or Business tier.
    ///
    /// **Options:**
    /// - `480p` - Supports only 5 or 10 second videos. Output: 24fps. Cost: 120 credits per 5 seconds.
    /// - `720p` - Supports videos between 5-60 seconds. Output: 30fps. Cost: 300 credits per 5 seconds.
    /// - `1080p` - Supports videos between 5-60 seconds. Output: 30fps. Cost: 600 credits per 5 seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<crate::models::V1TextToVideoCreateBodyResolutionEnum>,
    pub style: crate::models::V1TextToVideoCreateBodyStyle,
}
