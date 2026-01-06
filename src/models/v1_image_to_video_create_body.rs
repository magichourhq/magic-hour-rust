/// V1ImageToVideoCreateBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1ImageToVideoCreateBody {
    /// Provide the assets for image-to-video.
    pub assets: crate::models::V1ImageToVideoCreateBodyAssets,
    /// The total duration of the output video in seconds.
    pub end_seconds: f64,
    /// `height` is deprecated and no longer influences the output video's resolution.
    ///
    /// Output resolution is determined by the **minimum** of:
    /// - The resolution of the input video
    /// - The maximum resolution allowed by your subscription tier. See our [pricing page](https://magichour.ai/pricing) for more details.
    ///
    /// This field is retained only for backward compatibility and will be removed in a future release.
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub height: crate::core::patch::Patch<i64>,
    /// Give your video a custom name for easy identification.
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
    /// `width` is deprecated and no longer influences the output video's resolution.
    ///
    /// Output resolution is determined by the **minimum** of:
    /// - The resolution of the input video
    /// - The maximum resolution allowed by your subscription tier. See our [pricing page](https://magichour.ai/pricing) for more details.
    ///
    /// This field is retained only for backward compatibility and will be removed in a future release.
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub width: crate::core::patch::Patch<i64>,
}
