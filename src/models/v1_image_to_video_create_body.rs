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
    /// The name of video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Attributed used to dictate the style of the output
    pub style: crate::models::V1ImageToVideoCreateBodyStyle,
    /// This field does not affect the output video's resolution. The video's orientation will match that of the input image.
    ///
    /// It is retained solely for backward compatibility and will be deprecated in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}
