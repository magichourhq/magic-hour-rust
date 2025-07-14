/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    /// The total duration of the output video in seconds.
    pub end_seconds: f64,
    /// The name of video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Determines the orientation of the output video
    pub orientation: crate::models::V1TextToVideoCreateBodyOrientationEnum,
    /// Controls the output video resolution. Defaults to `720p` if not specified.
    ///
    /// **Options:**
    /// - `480p` - Supports only 5 or 10 second videos. Output: 24fps. Cost: 120 credits per 5 seconds.
    /// - `720p` - Supports videos between 5-60 seconds. Output: 30fps. Cost: 300 credits per 5 seconds.
    /// - `1080p` - Supports videos between 5-60 seconds. Output: 30fps. Cost: 600 credits per 5 seconds. **Requires** `pro` or `business` tier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<crate::models::V1TextToVideoCreateBodyResolutionEnum>,
    pub style: crate::models::V1TextToVideoCreateBodyStyle,
}
